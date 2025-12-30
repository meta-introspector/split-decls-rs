// Generated macro for impl_57 (impl)
macro_rules! Depcrate_unameimpl_57 {
() => {
// Module: crate::uname
// Provides: {"impl_57"}
// Dependencies: {}
impl UnameField { fn cli_arg_name (& self) -> & 'static str { match self { UnameField :: Sysname => "-s" , UnameField :: Release => "-r" , UnameField :: Version => "-v" , UnameField :: Machine => "-m" , UnameField :: Nodename => "-n" , UnameField :: OperatingSystem => "-o" , } } fn supports_uname_syscall (& self) -> bool { self != & UnameField :: OperatingSystem } fn get_from_syscall (& self) -> Option < String > { if ! self . supports_uname_syscall () { return None ; } let utsname = match nix_uname () { Ok (utsname) => utsname , Err (e) => { log :: error ! ("Failed to invoke native uname: {e:?}") ; return None ; } } ; let val_os = match self { UnameField :: Sysname => utsname . sysname () , UnameField :: Release => utsname . release () , UnameField :: Version => utsname . version () , UnameField :: Machine => utsname . machine () , UnameField :: Nodename => utsname . nodename () , UnameField :: OperatingSystem => return None , } ; let val = val_os . to_str () ; if val . is_none () { error ! ("Failed to convert uname value to string: {val_os:?}") ; return None ; } val . map (String :: from) } }
};
}
