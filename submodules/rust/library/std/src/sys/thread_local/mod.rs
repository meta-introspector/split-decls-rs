mkitem!{cfg_select ! { any (all (target_family = "wasm" , not (target_feature = "atomics")) , target_os = "uefi" , target_os = "zkvm" , target_os = "trusty" ,) => { mod no_threads ; pub use no_threads :: { EagerStorage , LazyStorage , thread_local_inner } ; pub (crate) use no_threads :: { LocalPointer , local_pointer } ; } target_thread_local => { mod native ; pub use native :: { EagerStorage , LazyStorage , thread_local_inner } ; pub (crate) use native :: { LocalPointer , local_pointer } ; } _ => { mod os ; pub use os :: { Storage , thread_local_inner } ; pub (crate) use os :: { LocalPointer , local_pointer } ; } }}
mkmod!{destructors, { 
                getname!(destructors);
                getsrc!(destructors);
                getpath!(destructors);
                get_deps!(destructors);
                get_crates!(destructors);
                mkinclude!(destructors);
                mkitem!{cfg_select ! { any (target_os = "linux" , target_os = "android" , target_os = "fuchsia" , target_os = "redox" , target_os = "hurd" , target_os = "netbsd" , target_os = "dragonfly") => { mod linux_like ; mod list ; pub (super) use linux_like :: register ; pub (super) use list :: run ; } _ => { mod list ; pub (super) use list :: register ; pub (crate) use list :: run ; } }} 
            }}
mkmod!{guard, { 
                getname!(guard);
                getsrc!(guard);
                getpath!(guard);
                get_deps!(guard);
                get_crates!(guard);
                mkinclude!(guard);
                mkitem!{cfg_select ! { all (target_thread_local , target_vendor = "apple") => { mod apple ; pub (crate) use apple :: enable ; } target_os = "windows" => { mod windows ; pub (crate) use windows :: enable ; } any (all (target_family = "wasm" , not (all (target_os = "wasi" , target_env = "p1" , target_feature = "atomics"))) , target_os = "uefi" , target_os = "zkvm" , target_os = "trusty" ,) => { pub (crate) fn enable () { # [cfg (all (target_family = "wasm" , target_feature = "atomics"))] # [allow (unused)] use super :: destructors :: run ; # [allow (unused)] use crate :: rt :: thread_cleanup ; } } any (target_os = "hermit" , target_os = "xous" ,) => { pub (crate) fn enable () { } } target_os = "solid_asp3" => { mod solid ; pub (crate) use solid :: enable ; } _ => { mod key ; pub (crate) use key :: enable ; } }} 
            }}
mkmod!{key, { 
                getname!(key);
                getsrc!(key);
                getpath!(key);
                get_deps!(key);
                get_crates!(key);
                mkinclude!(key);
                mkitem!{cfg_select ! { any (all (not (target_vendor = "apple") , not (target_family = "wasm") , target_family = "unix" ,) , all (not (target_thread_local) , target_vendor = "apple") , target_os = "teeos" , all (target_os = "wasi" , target_env = "p1" , target_feature = "atomics") ,) => { mod racy ; mod unix ; # [cfg (test)] mod tests ; pub (super) use racy :: LazyKey ; pub (super) use unix :: { Key , set } ; # [cfg (any (not (target_thread_local) , test))] pub (super) use unix :: get ; use unix :: { create , destroy } ; } all (not (target_thread_local) , target_os = "windows") => { # [cfg (test)] mod tests ; mod windows ; pub (super) use windows :: { Key , LazyKey , get , run_dtors , set } ; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod racy ; mod sgx ; # [cfg (test)] mod tests ; pub (super) use racy :: LazyKey ; pub (super) use sgx :: { Key , get , set } ; use sgx :: { create , destroy } ; } target_os = "xous" => { mod racy ; # [cfg (test)] mod tests ; mod xous ; pub (super) use racy :: LazyKey ; pub (crate) use xous :: destroy_tls ; pub (super) use xous :: { Key , get , set } ; use xous :: { create , destroy } ; } _ => { } }} 
            }}

macro_rules! abort_on_dtor_unwind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_on_dtor_unwind in module {}", module_path!());
    };
}

mkfn!{
    abort_on_dtor_unwind_introspect!();
    # [doc = " Run a callback in a scenario which must not unwind (such as a `extern \"C\""] # [doc = " fn` declared in a user crate). If the callback unwinds anyway, then"] # [doc = " `rtabort` with a message about thread local panicking on drop."] # [inline] # [allow (dead_code)] fn abort_on_dtor_unwind (f : impl FnOnce ()) { let guard = DtorUnwindGuard ; f () ; core :: mem :: forget (guard) ; struct DtorUnwindGuard ; impl Drop for DtorUnwindGuard { # [inline] fn drop (& mut self) { rtabort ! ("thread local panicked on drop") ; } } }
}