// Generated macro for macro_15 (macro)
macro_rules! Depcratemacro_15 {
() => {
// Module: crate
// Provides: {"macro_15"}
// Dependencies: {}
define_signal_enum ! { # [doc = " The signal types that we are able to listen for."] pub enum Signal { # [doc = " `SIGHUP`"] Hup = SIGHUP , # [doc = " `SIGINT`"] Int = SIGINT , # [doc = " `SIGQUIT`"] Quit = SIGQUIT , # [doc = " `SIGILL`"] Ill = SIGILL , # [doc = " `SIGTRAP`"] Trap = SIGTRAP , # [doc = " `SIGABRT`, aka `SIGIOT`"] # [doc (alias = "Iot")] # [doc (alias = "Abrt")] Abort = SIGABRT , # [doc = " `SIGBUS`"] Bus = SIGBUS , # [doc = " `SIGFPE`"] Fpe = SIGFPE , # [doc = " `SIGKILL`"] Kill = SIGKILL , # [doc = " `SIGUSR1`"] Usr1 = SIGUSR1 , # [doc = " `SIGSEGV`"] Segv = SIGSEGV , # [doc = " `SIGUSR2`"] Usr2 = SIGUSR2 , # [doc = " `SIGPIPE`"] Pipe = SIGPIPE , # [doc = " `SIGALRM`"] # [doc (alias = "Alrm")] Alarm = SIGALRM , # [doc = " `SIGTERM`"] Term = SIGTERM , # [doc = " `SIGCHLD`"] # [doc (alias = "Chld")] Child = SIGCHLD , # [doc = " `SIGCONT`"] Cont = SIGCONT , # [doc = " `SIGSTOP`"] Stop = SIGSTOP , # [doc = " `SIGTSTP`"] Tstp = SIGTSTP , # [doc = " `SIGTTIN`"] Ttin = SIGTTIN , # [doc = " `SIGTTOU`"] Ttou = SIGTTOU , # [doc = " `SIGURG`"] Urg = SIGURG , # [doc = " `SIGXCPU`"] Xcpu = SIGXCPU , # [doc = " `SIGXFSZ`"] Xfsz = SIGXFSZ , # [doc = " `SIGVTALRM`"] # [doc (alias = "Vtalrm")] Vtalarm = SIGVTALRM , # [doc = " `SIGPROF`"] Prof = SIGPROF , # [doc = " `SIGWINCH`"] Winch = SIGWINCH , # [doc = " `SIGIO`, aka `SIGPOLL`"] # [doc (alias = "Poll")] Io = SIGIO , # [doc = " `SIGSYS`, aka `SIGUNUSED`"] # [doc (alias = "Unused")] Sys = SIGSYS , } }
};
}
