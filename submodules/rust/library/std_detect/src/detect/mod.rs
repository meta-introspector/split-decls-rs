mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                 
            }}
mkuse!{# [doc (hidden)] # [unstable (feature = "stdarch_internal" , issue = "none")] pub use self :: arch :: __is_feature_detected ;}
mkuse!{pub (crate) use self :: arch :: Feature ;}
mkmod!{bit, { 
                getname!(bit);
                getsrc!(bit);
                getpath!(bit);
                get_deps!(bit);
                get_crates!(bit);
                mkinclude!(bit);
                 
            }}
mkmod!{cache, { 
                getname!(cache);
                getsrc!(cache);
                getpath!(cache);
                get_deps!(cache);
                get_crates!(cache);
                mkinclude!(cache);
                 
            }}
mkitem!{cfg_select ! { miri => { # [path = "os/other.rs"] mod os ; } any (target_arch = "x86" , target_arch = "x86_64") => { # [path = "os/x86.rs"] mod os ; } all (any (target_os = "linux" , target_os = "android") , feature = "libc") => { # [cfg (any (target_arch = "riscv32" , target_arch = "riscv64"))] # [path = "os/riscv.rs"] mod riscv ; # [path = "os/linux/mod.rs"] mod os ; } all (target_os = "freebsd" , feature = "libc") => { # [cfg (target_arch = "aarch64")] # [path = "os/aarch64.rs"] mod aarch64 ; # [path = "os/freebsd/mod.rs"] mod os ; } all (target_os = "openbsd" , target_arch = "aarch64" , feature = "libc") => { # [allow (dead_code)] # [path = "os/aarch64.rs"] mod aarch64 ; # [path = "os/openbsd/aarch64.rs"] mod os ; } all (target_os = "windows" , any (target_arch = "aarch64" , target_arch = "arm64ec")) => { # [path = "os/windows/aarch64.rs"] mod os ; } all (target_vendor = "apple" , target_arch = "aarch64" , feature = "libc") => { # [path = "os/darwin/aarch64.rs"] mod os ; } _ => { # [path = "os/other.rs"] mod os ; } }}

macro_rules! check_for_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_for in module {}", module_path!());
    };
}

mkfn!{
    check_for_introspect!();
    # [doc = " Performs run-time feature detection."] # [inline] # [allow (dead_code)] fn check_for (x : Feature) -> bool { cache :: test (x as u32) }
}

macro_rules! features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function features in module {}", module_path!());
    };
}

mkfn!{
    features_introspect!();
    # [doc = " Returns an `Iterator<Item=(&'static str, bool)>` where"] # [doc = " `Item.0` is the feature name, and `Item.1` is a `bool` which"] # [doc = " is `true` if the feature is supported by the host and `false` otherwise."] # [unstable (feature = "stdarch_internal" , issue = "none")] pub fn features () -> impl Iterator < Item = (& 'static str , bool) > { cfg_select ! { any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "arm" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "mips" , target_arch = "mips64" , target_arch = "loongarch32" , target_arch = "loongarch64" , target_arch = "s390x" ,) => { (0_u8 .. Feature :: _last as u8) . map (| discriminant : u8 | { # [allow (bindings_with_variant_name)] let f : Feature = unsafe { core :: mem :: transmute (discriminant) } ; let name : &'static str = f . to_str () ; let enabled : bool = check_for (f) ; (name , enabled) }) } _ => None . into_iter () , } }
}