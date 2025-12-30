// Generated macro for signum (module)
macro_rules! Depcratesignum {
() => {
// Module: crate
// Provides: {"signum"}
// Dependencies: {}
mod signum { pub (crate) use std :: os :: raw :: c_int ; macro_rules ! sig { ($ rustix_name : ident , $ raw_value : literal) => { { # [cfg (unix)] { rustix :: process :: Signal ::$ rustix_name . as_raw () } # [cfg (windows)] { $ raw_value } } } ; } pub const SIGHUP : c_int = sig ! (HUP , 1) ; pub const SIGINT : c_int = sig ! (INT , 2) ; pub const SIGQUIT : c_int = sig ! (QUIT , 3) ; pub const SIGILL : c_int = sig ! (ILL , 4) ; pub const SIGTRAP : c_int = sig ! (TRAP , 5) ; pub const SIGABRT : c_int = sig ! (ABORT , 6) ; pub const SIGFPE : c_int = sig ! (FPE , 8) ; pub const SIGKILL : c_int = sig ! (KILL , 9) ; pub const SIGSEGV : c_int = sig ! (SEGV , 11) ; pub const SIGPIPE : c_int = sig ! (PIPE , 13) ; pub const SIGALRM : c_int = sig ! (ALARM , 14) ; pub const SIGTERM : c_int = sig ! (TERM , 15) ; pub const SIGTTIN : c_int = sig ! (TTIN , 21) ; pub const SIGTTOU : c_int = sig ! (TTOU , 22) ; pub const SIGXCPU : c_int = sig ! (XCPU , 24) ; pub const SIGXFSZ : c_int = sig ! (XFSZ , 25) ; pub const SIGVTALRM : c_int = sig ! (VTALARM , 26) ; pub const SIGPROF : c_int = sig ! (PROF , 27) ; pub const SIGWINCH : c_int = sig ! (WINCH , 28) ; pub const SIGCHLD : c_int = sig ! (CHILD , 17) ; pub const SIGBUS : c_int = sig ! (BUS , 7) ; pub const SIGUSR1 : c_int = sig ! (USR1 , 10) ; pub const SIGUSR2 : c_int = sig ! (USR2 , 12) ; pub const SIGCONT : c_int = sig ! (CONT , 18) ; pub const SIGSTOP : c_int = sig ! (STOP , 19) ; pub const SIGTSTP : c_int = sig ! (TSTP , 20) ; pub const SIGURG : c_int = sig ! (URG , 23) ; pub const SIGIO : c_int = sig ! (IO , 29) ; pub const SIGSYS : c_int = sig ! (SYS , 31) ; }
};
}
