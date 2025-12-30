// Generated macro for SyscallInterface (trait)
macro_rules! Depcrate_syscalls_interfacesSyscallInterface {
() => {
// Module: crate::syscalls::interfaces
// Provides: {"SyscallInterface"}
// Dependencies: {}
pub trait SyscallInterface : Send + Sync { fn init (& self) { } fn get_application_parameters (& self) -> (i32 , * const * const u8 , * const * const u8) { let mut argv = Vec :: new () ; let name = Box :: leak (Box :: new ("bin\0")) . as_ptr () ; argv . push (name) ; let args = env :: args () ; debug ! ("Setting argv as: {args:?}") ; for arg in args { let ptr = Box :: leak (format ! ("{arg}\0") . into_boxed_str ()) . as_ptr () ; argv . push (ptr) ; } let mut envv = Vec :: new () ; let envs = env :: vars () ; debug ! ("Setting envv as: {envs:?}") ; for (key , value) in envs { let ptr = Box :: leak (format ! ("{key}={value}\0") . into_boxed_str ()) . as_ptr () ; envv . push (ptr) ; } envv . push (core :: ptr :: null :: < u8 > ()) ; let argc = argv . len () as i32 ; let argv = argv . leak () . as_ptr () ; let envv = if envv . len () == 1 { core :: ptr :: null :: < * const u8 > () } else { envv . leak () . as_ptr () } ; (argc , argv , envv) } fn shutdown (& self , error_code : i32) -> ! { panic_println ! ("exit status {error_code}") ; arch :: processor :: shutdown (error_code) } }
};
}
