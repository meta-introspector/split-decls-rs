mkuse!{use std :: sync :: { Arc , LazyLock , OnceLock } ;}
mkuse!{pub use jobserver_crate :: { Acquired , Client , HelperThread } ;}
mkuse!{use jobserver_crate :: { FromEnv , FromEnvErrorKind } ;}
mkuse!{use parking_lot :: { Condvar , Mutex } ;}
mkitem!{static GLOBAL_CLIENT : LazyLock < Result < Client , String > > = LazyLock :: new (| | { let FromEnv { client , var } = unsafe { Client :: from_env_ext (true) } ; let error = match client { Ok (client) => return Ok (client) , Err (e) => e , } ; if matches ! (error . kind () , FromEnvErrorKind :: NoEnvVar | FromEnvErrorKind :: NoJobserver | FromEnvErrorKind :: NegativeFd | FromEnvErrorKind :: Unsupported) { return Ok (default_client ()) ; } let (name , value) = var . unwrap () ; Err (format ! ("failed to connect to jobserver from environment variable `{name}={:?}`: {error}" , value)) }) ;}

macro_rules! default_client_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_client in module {}", module_path!());
    };
}

mkfn!{
    default_client_introspect!();
    fn default_client () -> Client { let client = Client :: new (32) . expect ("failed to create jobserver") ; client . acquire_raw () . ok () ; client }
}
mkitem!{static GLOBAL_CLIENT_CHECKED : OnceLock < Client > = OnceLock :: new () ;}

macro_rules! initialize_checked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function initialize_checked in module {}", module_path!());
    };
}

mkfn!{
    initialize_checked_introspect!();
    pub fn initialize_checked (report_warning : impl FnOnce (& 'static str)) { let client_checked = match & * GLOBAL_CLIENT { Ok (client) => client . clone () , Err (e) => { report_warning (e) ; default_client () } } ; GLOBAL_CLIENT_CHECKED . set (client_checked) . ok () ; }
}
mkitem!{const ACCESS_ERROR : & str = "jobserver check should have been called earlier" ;}

macro_rules! client_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function client in module {}", module_path!());
    };
}

mkfn!{
    client_introspect!();
    pub fn client () -> Client { GLOBAL_CLIENT_CHECKED . get () . expect (ACCESS_ERROR) . clone () }
}
mkitem!{mkstruct!{struct ProxyData { # [doc = " The number of tokens assigned to threads."] # [doc = " If this is 0, a single token is still assigned to this process, but is unused."] used : u16 , # [doc = " The number of threads requesting a token"] pending : u16 , }}}
mkitem!{mkstruct!{# [doc = " This is a jobserver proxy used to ensure that we hold on to at least one token."] pub struct Proxy { client : Client , data : Mutex < ProxyData > , # [doc = " Threads which are waiting on a token will wait on this."] wake_pending : Condvar , helper : OnceLock < HelperThread > , }}}
mkitem!{mkimpl!{impl Proxy { pub fn new () -> Arc < Self > { let proxy = Arc :: new (Proxy { client : client () , data : Mutex :: new (ProxyData { used : 1 , pending : 0 }) , wake_pending : Condvar :: new () , helper : OnceLock :: new () , }) ; let proxy_ = Arc :: clone (& proxy) ; let helper = proxy . client . clone () . into_helper_thread (move | token | { if let Ok (token) = token { let mut data = proxy_ . data . lock () ; if data . pending > 0 { token . drop_without_releasing () ; assert ! (data . used > 0) ; data . used += 1 ; data . pending -= 1 ; proxy_ . wake_pending . notify_one () ; } else { drop (data) ; drop (token) ; } } }) . expect ("failed to create helper thread") ; proxy . helper . set (helper) . unwrap () ; proxy } pub fn acquire_thread (& self) { let mut data = self . data . lock () ; if data . used == 0 { assert_eq ! (data . pending , 0) ; data . used += 1 ; } else { self . helper . get () . unwrap () . request_token () ; data . pending += 1 ; self . wake_pending . wait (& mut data) ; } } pub fn release_thread (& self) { let mut data = self . data . lock () ; if data . pending > 0 { data . pending -= 1 ; self . wake_pending . notify_one () ; } else { data . used -= 1 ; if data . used > 0 { drop (data) ; self . client . release_raw () . ok () ; } } } }}}