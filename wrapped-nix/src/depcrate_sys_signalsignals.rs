// Generated macro for SIGNALS (const)
macro_rules! Depcrate_sys_signalSIGNALS {
() => {
// Module: crate::sys::signal
// Provides: {"SIGNALS"}
// Dependencies: {}
# [cfg (not (any (linux_android , target_os = "fuchsia" , target_os = "emscripten" , target_os = "aix" , target_os = "redox" , target_os = "haiku" , target_os = "solaris" , target_os = "cygwin")))] # [cfg (feature = "signal")] const SIGNALS : [Signal ; 31] = [SIGHUP , SIGINT , SIGQUIT , SIGILL , SIGTRAP , SIGABRT , SIGBUS , SIGFPE , SIGKILL , SIGUSR1 , SIGSEGV , SIGUSR2 , SIGPIPE , SIGALRM , SIGTERM , SIGCHLD , SIGCONT , SIGSTOP , SIGTSTP , SIGTTIN , SIGTTOU , SIGURG , SIGXCPU , SIGXFSZ , SIGVTALRM , SIGPROF , SIGWINCH , SIGIO , SIGSYS , SIGEMT , SIGINFO ,] ;
};
}
