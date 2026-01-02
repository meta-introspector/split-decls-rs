mkuse!{pub use super :: common :: Env ;}
mkuse!{use crate :: collections :: HashMap ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicUsize , Ordering } ;}
mkuse!{use crate :: sync :: { Mutex , Once } ;}
mkuse!{use crate :: sys :: pal :: os :: { get_application_parameters , params } ;}
mkitem!{static ENV : Atomic < usize > = AtomicUsize :: new (0) ;}
mkitem!{static ENV_INIT : Once = Once :: new () ;}
mkitem!{type EnvStore = Mutex < HashMap < OsString , OsString > > ;}

macro_rules! get_env_store_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_env_store in module {}", module_path!());
    };
}

mkfn!{
    get_env_store_introspect!();
    fn get_env_store () -> & 'static EnvStore { ENV_INIT . call_once (| | { let env_store = EnvStore :: default () ; if let Some (params) = get_application_parameters () { for param in params { if let Ok (envs) = params :: EnvironmentBlock :: try_from (& param) { let mut env_store = env_store . lock () . unwrap () ; for env in envs { env_store . insert (env . key . into () , env . value . into ()) ; } break ; } } } ENV . store (Box :: into_raw (Box :: new (env_store)) as _ , Ordering :: Relaxed) }) ; unsafe { & * core :: ptr :: with_exposed_provenance :: < EnvStore > (ENV . load (Ordering :: Relaxed)) } }
}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    pub fn env () -> Env { let clone_to_vec = | map : & HashMap < OsString , OsString > | -> Vec < _ > { map . iter () . map (| (k , v) | (k . clone () , v . clone ())) . collect () } ; let env = clone_to_vec (& * get_env_store () . lock () . unwrap ()) ; Env :: new (env) }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (k : & OsStr) -> Option < OsString > { get_env_store () . lock () . unwrap () . get (k) . cloned () }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (k : & OsStr , v : & OsStr) -> io :: Result < () > { let (k , v) = (k . to_owned () , v . to_owned ()) ; get_env_store () . lock () . unwrap () . insert (k , v) ; Ok (()) }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (k : & OsStr) -> io :: Result < () > { get_env_store () . lock () . unwrap () . remove (k) ; Ok (()) }
}