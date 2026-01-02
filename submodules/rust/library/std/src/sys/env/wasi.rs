mkuse!{use core :: slice :: memchr ;}
mkuse!{pub use super :: common :: Env ;}
mkuse!{use crate :: ffi :: { CStr , OsStr , OsString } ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: os :: wasi :: prelude :: * ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_with_cstr ;}
mkuse!{use crate :: sys :: pal :: os :: { cvt , libc } ;}
mkitem!{cfg_select ! { target_feature = "atomics" => { use crate :: sync :: { PoisonError , RwLock } ; static ENV_LOCK : RwLock < () > = RwLock :: new (()) ; pub fn env_read_lock () -> impl Drop { ENV_LOCK . read () . unwrap_or_else (PoisonError :: into_inner) } pub fn env_write_lock () -> impl Drop { ENV_LOCK . write () . unwrap_or_else (PoisonError :: into_inner) } } _ => { pub fn env_read_lock () -> impl Drop { Box :: new (()) } pub fn env_write_lock () -> impl Drop { Box :: new (()) } } }}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    pub fn env () -> Env { unsafe { let _guard = env_read_lock () ; let mut environ = libc :: __wasilibc_get_environ () ; let mut result = Vec :: new () ; if ! environ . is_null () { while ! (* environ) . is_null () { if let Some (key_value) = parse (CStr :: from_ptr (* environ) . to_bytes ()) { result . push (key_value) ; } environ = environ . add (1) ; } } return Env :: new (result) ; } fn parse (input : & [u8]) -> Option < (OsString , OsString) > { if input . is_empty () { return None ; } let pos = memchr :: memchr (b'=' , & input [1 ..]) . map (| p | p + 1) ; pos . map (| p | { (OsStringExt :: from_vec (input [.. p] . to_vec ()) , OsStringExt :: from_vec (input [p + 1 ..] . to_vec ()) ,) }) } }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (k : & OsStr) -> Option < OsString > { run_with_cstr (k . as_bytes () , & | k | { let _guard = env_read_lock () ; let v = unsafe { libc :: getenv (k . as_ptr ()) } as * const libc :: c_char ; if v . is_null () { Ok (None) } else { let bytes = unsafe { CStr :: from_ptr (v) } . to_bytes () . to_vec () ; Ok (Some (OsStringExt :: from_vec (bytes))) } }) . ok () . flatten () }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (k : & OsStr , v : & OsStr) -> io :: Result < () > { run_with_cstr (k . as_bytes () , & | k | { run_with_cstr (v . as_bytes () , & | v | unsafe { let _guard = env_write_lock () ; cvt (libc :: setenv (k . as_ptr () , v . as_ptr () , 1)) . map (drop) }) }) }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (n : & OsStr) -> io :: Result < () > { run_with_cstr (n . as_bytes () , & | nbuf | unsafe { let _guard = env_write_lock () ; cvt (libc :: unsetenv (nbuf . as_ptr ())) . map (drop) }) }
}