mkuse!{use core :: slice :: memchr ;}
mkuse!{pub use super :: common :: Env ;}
mkuse!{use crate :: collections :: HashMap ;}
mkuse!{use crate :: ffi :: { CStr , OsStr , OsString , c_char } ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: os :: hermit :: ffi :: OsStringExt ;}
mkuse!{use crate :: sync :: Mutex ;}
mkitem!{static ENV : Mutex < Option < HashMap < OsString , OsString > > > = Mutex :: new (None) ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub fn init (env : * const * const c_char) { let mut guard = ENV . lock () . unwrap () ; let map = guard . insert (HashMap :: new ()) ; if env . is_null () { return ; } unsafe { let mut environ = env ; while ! (* environ) . is_null () { if let Some ((key , value)) = parse (CStr :: from_ptr (* environ) . to_bytes ()) { map . insert (key , value) ; } environ = environ . add (1) ; } } fn parse (input : & [u8]) -> Option < (OsString , OsString) > { if input . is_empty () { return None ; } let pos = memchr :: memchr (b'=' , & input [1 ..]) . map (| p | p + 1) ; pos . map (| p | { (OsStringExt :: from_vec (input [.. p] . to_vec ()) , OsStringExt :: from_vec (input [p + 1 ..] . to_vec ()) ,) }) } }
}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    # [doc = " Returns a vector of (variable, value) byte-vector pairs for all the"] # [doc = " environment variables of the current process."] pub fn env () -> Env { let guard = ENV . lock () . unwrap () ; let env = guard . as_ref () . unwrap () ; let result = env . iter () . map (| (key , value) | (key . clone () , value . clone ())) . collect () ; Env :: new (result) }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (k : & OsStr) -> Option < OsString > { ENV . lock () . unwrap () . as_ref () . unwrap () . get (k) . cloned () }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (k : & OsStr , v : & OsStr) -> io :: Result < () > { let (k , v) = (k . to_owned () , v . to_owned ()) ; ENV . lock () . unwrap () . as_mut () . unwrap () . insert (k , v) ; Ok (()) }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (k : & OsStr) -> io :: Result < () > { ENV . lock () . unwrap () . as_mut () . unwrap () . remove (k) ; Ok (()) }
}