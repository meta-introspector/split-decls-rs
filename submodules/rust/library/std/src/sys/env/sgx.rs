mkuse!{pub use super :: common :: Env ;}
mkuse!{use crate :: collections :: HashMap ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicUsize , Ordering } ;}
mkuse!{use crate :: sync :: { Mutex , Once } ;}
mkitem!{# [cfg_attr (test , linkage = "available_externally")] # [unsafe (export_name = "_ZN16__rust_internals3std3sys3pal3sgx2os3ENVE")] static ENV : Atomic < usize > = AtomicUsize :: new (0) ;}
mkitem!{# [cfg_attr (test , linkage = "available_externally")] # [unsafe (export_name = "_ZN16__rust_internals3std3sys3pal3sgx2os8ENV_INITE")] static ENV_INIT : Once = Once :: new () ;}
mkitem!{type EnvStore = Mutex < HashMap < OsString , OsString > > ;}

macro_rules! get_env_store_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_env_store in module {}", module_path!());
    };
}

mkfn!{
    get_env_store_introspect!();
    fn get_env_store () -> Option < & 'static EnvStore > { unsafe { (ENV . load (Ordering :: Relaxed) as * const EnvStore) . as_ref () } }
}

macro_rules! create_env_store_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_env_store in module {}", module_path!());
    };
}

mkfn!{
    create_env_store_introspect!();
    fn create_env_store () -> & 'static EnvStore { ENV_INIT . call_once (| | { ENV . store (Box :: into_raw (Box :: new (EnvStore :: default ())) as _ , Ordering :: Relaxed) }) ; unsafe { & * (ENV . load (Ordering :: Relaxed) as * const EnvStore) } }
}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    pub fn env () -> Env { let clone_to_vec = | map : & HashMap < OsString , OsString > | -> Vec < _ > { map . iter () . map (| (k , v) | (k . clone () , v . clone ())) . collect () } ; let env = get_env_store () . map (| env | clone_to_vec (& env . lock () . unwrap ())) . unwrap_or_default () ; Env :: new (env) }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (k : & OsStr) -> Option < OsString > { get_env_store () . and_then (| s | s . lock () . unwrap () . get (k) . cloned ()) }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (k : & OsStr , v : & OsStr) -> io :: Result < () > { let (k , v) = (k . to_owned () , v . to_owned ()) ; create_env_store () . lock () . unwrap () . insert (k , v) ; Ok (()) }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (k : & OsStr) -> io :: Result < () > { if let Some (env) = get_env_store () { env . lock () . unwrap () . remove (k) ; } Ok (()) }
}