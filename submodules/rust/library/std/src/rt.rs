mkuse!{# [rustfmt :: skip] pub use crate :: panicking :: { begin_panic , panic_count } ;}
mkuse!{pub use core :: panicking :: { panic_display , panic_fmt } ;}
mkuse!{# [rustfmt :: skip] use crate :: any :: Any ;}
mkuse!{use crate :: sync :: Once ;}
mkuse!{use crate :: thread :: { self , main_thread } ;}
mkuse!{use crate :: { mem , panic , sys } ;}

macro_rules! __rust_abort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_abort in module {}", module_path!());
    };
}

mkfn!{
    __rust_abort_introspect!();
    # [cfg (not (test))] # [rustc_std_internal_symbol] fn __rust_abort () { crate :: process :: abort () ; }
}
mkitem!{macro_rules ! rtprintpanic { ($ ($ t : tt) *) => { # [cfg (not (feature = "panic_immediate_abort"))] if let Some (mut out) = crate :: sys :: stdio :: panic_output () { let _ = crate :: io :: Write :: write_fmt (& mut out , format_args ! ($ ($ t) *)) ; } # [cfg (feature = "panic_immediate_abort")] { let _ = format_args ! ($ ($ t) *) ; } } }}
mkitem!{macro_rules ! rtabort { ($ ($ t : tt) *) => { { rtprintpanic ! ("fatal runtime error: {}, aborting\n" , format_args ! ($ ($ t) *)) ; crate :: process :: abort () ; } } }}
mkitem!{macro_rules ! rtassert { ($ e : expr) => { if !$ e { rtabort ! (concat ! ("assertion failed: " , stringify ! ($ e))) ; } } ; }}
mkitem!{macro_rules ! rtunwrap { ($ ok : ident , $ e : expr) => { match $ e { $ ok (v) => v , ref err => { let err = err . as_ref () . map (drop) ; rtabort ! (concat ! ("unwrap failed: " , stringify ! ($ e) , " = {:?}") , err) } } } ; }}

macro_rules! handle_rt_panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_rt_panic in module {}", module_path!());
    };
}

mkfn!{
    handle_rt_panic_introspect!();
    fn handle_rt_panic < T > (e : Box < dyn Any + Send >) -> T { mem :: forget (e) ; rtabort ! ("initialization or cleanup bug") ; }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [cfg_attr (test , allow (dead_code))] unsafe fn init (argc : isize , argv : * const * const u8 , sigpipe : u8) { # [cfg_attr (target_os = "teeos" , allow (unused_unsafe))] unsafe { sys :: init (argc , argv , sigpipe) } ; unsafe { main_thread :: set (thread :: current_id ()) } ; }
}

macro_rules! thread_cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function thread_cleanup in module {}", module_path!());
    };
}

mkfn!{
    thread_cleanup_introspect!();
    # [doc = " Clean up the thread-local runtime state. This *should* be run after all other"] # [doc = " code managed by the Rust runtime, but will not cause UB if that condition is"] # [doc = " not fulfilled. Also note that this function is not guaranteed to be run, but"] # [doc = " skipping it will cause leaks and therefore is to be avoided."] pub (crate) fn thread_cleanup () { panic :: catch_unwind (| | { crate :: thread :: drop_current () ; }) . unwrap_or_else (handle_rt_panic) ; }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub (crate) fn cleanup () { static CLEANUP : Once = Once :: new () ; CLEANUP . call_once (| | unsafe { crate :: io :: cleanup () ; sys :: cleanup () ; }) ; }
}

macro_rules! lang_start_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lang_start_internal in module {}", module_path!());
    };
}

mkfn!{
    lang_start_internal_introspect!();
    # [cfg (not (test))] fn lang_start_internal (main : & (dyn Fn () -> i32 + Sync + crate :: panic :: RefUnwindSafe) , argc : isize , argv : * const * const u8 , sigpipe : u8 ,) -> isize { panic :: catch_unwind (move | | { unsafe { init (argc , argv , sigpipe) } ; let ret_code = panic :: catch_unwind (main) . unwrap_or_else (move | payload | { let payload = panic :: AssertUnwindSafe (payload) ; panic :: catch_unwind (move | | drop ({ payload } . 0)) . unwrap_or_else (move | e | { mem :: forget (e) ; rtabort ! ("drop of the panic payload panicked") ; }) ; 101 }) ; let ret_code = ret_code as isize ; cleanup () ; crate :: sys :: exit_guard :: unique_thread_exit () ; ret_code }) . unwrap_or_else (handle_rt_panic) }
}

macro_rules! lang_start_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lang_start in module {}", module_path!());
    };
}

mkfn!{
    lang_start_introspect!();
    # [cfg (not (any (test , doctest)))] # [lang = "start"] fn lang_start < T : crate :: process :: Termination + 'static > (main : fn () -> T , argc : isize , argv : * const * const u8 , sigpipe : u8 ,) -> isize { lang_start_internal (& move | | crate :: sys :: backtrace :: __rust_begin_short_backtrace (main) . report () . to_i32 () , argc , argv , sigpipe ,) }
}