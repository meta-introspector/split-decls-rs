mkuse!{pub use super :: common :: Args ;}
mkuse!{use crate :: ffi :: CStr ;}
mkuse!{# [cfg (target_os = "hermit")] use crate :: os :: hermit :: ffi :: OsStringExt ;}
mkuse!{# [cfg (not (target_os = "hermit"))] use crate :: os :: unix :: ffi :: OsStringExt ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [doc = " One-time global initialization."] pub unsafe fn init (argc : isize , argv : * const * const u8) { unsafe { imp :: init (argc , argv) } }
}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    # [doc = " Returns the command line arguments"] pub fn args () -> Args { let (argc , argv) = imp :: argc_argv () ; let mut vec = Vec :: with_capacity (argc as usize) ; for i in 0 .. argc { let ptr = unsafe { argv . offset (i) . read () } ; if ptr . is_null () { break ; } let cstr = unsafe { CStr :: from_ptr (ptr) } ; vec . push (OsStringExt :: from_vec (cstr . to_bytes () . to_vec ())) ; } Args :: new (vec) }
}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                mkuse!{use crate :: ffi :: c_char ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicIsize , AtomicPtr , Ordering } ;}
mkitem!{static ARGC : Atomic < isize > = AtomicIsize :: new (0) ;}
mkitem!{static ARGV : Atomic < * mut * const u8 > = AtomicPtr :: new (ptr :: null_mut ()) ;}

macro_rules! really_init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function really_init in module {}", module_path!());
    };
}

mkfn!{
    really_init_introspect!();
    unsafe fn really_init (argc : isize , argv : * const * const u8) { ARGC . store (argc , Ordering :: Relaxed) ; ARGV . store (argv as * mut _ , Ordering :: Relaxed) ; }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [inline (always)] pub unsafe fn init (argc : isize , argv : * const * const u8) { unsafe { really_init (argc , argv) } ; }
}
mkitem!{# [doc = " glibc passes argc, argv, and envp to functions in .init_array, as a non-standard extension."] # [doc = " This allows `std::env::args` to work even in a `cdylib`, as it does on macOS and Windows."] # [cfg (all (target_os = "linux" , target_env = "gnu"))] # [used] # [unsafe (link_section = ".init_array.00099")] static ARGV_INIT_ARRAY : extern "C" fn (crate :: os :: raw :: c_int , * const * const u8 , * const * const u8 ,) = { extern "C" fn init_wrapper (argc : crate :: os :: raw :: c_int , argv : * const * const u8 , _envp : * const * const u8 ,) { unsafe { really_init (argc as isize , argv) } ; } init_wrapper } ;}

macro_rules! argc_argv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function argc_argv in module {}", module_path!());
    };
}

mkfn!{
    argc_argv_introspect!();
    pub fn argc_argv () -> (isize , * const * const c_char) { let argv = ARGV . load (Ordering :: Relaxed) ; let argc = if argv . is_null () { 0 } else { ARGC . load (Ordering :: Relaxed) } ; (argc , argv . cast ()) }
} 
            }}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                mkuse!{use crate :: ffi :: { c_char , c_int } ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub unsafe fn init (_argc : isize , _argv : * const * const u8) { }
}

macro_rules! argc_argv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function argc_argv in module {}", module_path!());
    };
}

mkfn!{
    argc_argv_introspect!();
    pub fn argc_argv () -> (isize , * const * const c_char) { unsafe extern "C" { fn _NSGetArgc () -> * mut c_int ; fn _NSGetArgv () -> * mut * mut * mut c_char ; } let argc = unsafe { _NSGetArgc () . read () } ; let argv = unsafe { _NSGetArgv () . read () } ; (argc as isize , argv . cast ()) }
} 
            }}