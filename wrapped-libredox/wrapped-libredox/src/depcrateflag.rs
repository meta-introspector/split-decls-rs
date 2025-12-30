// Generated macro for flag (module)
macro_rules! Depcrateflag {
() => {
// Module: crate
// Provides: {"flag"}
// Dependencies: {}
pub mod flag { pub use libc :: { O_ACCMODE , O_APPEND , O_ASYNC , O_CLOEXEC , O_CREAT , O_DIRECTORY , O_EXCL , O_FSYNC , O_NOFOLLOW , O_NONBLOCK , O_PATH , O_RDONLY , O_RDWR , O_TRUNC , O_WRONLY , } ; pub use libc :: { CLOCK_MONOTONIC , CLOCK_REALTIME } ; pub use libc :: { SIG_BLOCK , SIG_SETMASK , SIG_UNBLOCK } ; pub use libc :: { SIGABRT , SIGALRM , SIGBUS , SIGCHLD , SIGCONT , SIGFPE , SIGHUP , SIGILL , SIGINT , SIGIO , SIGKILL , SIGPIPE , SIGPROF , SIGPWR , SIGQUIT , SIGSEGV , SIGSTKFLT , SIGSYS , SIGTERM , SIGTRAP , SIGTSTP , SIGTTIN , SIGTTOU , SIGURG , SIGUSR1 , SIGUSR2 , SIGVTALRM , SIGWINCH , SIGXFSZ , } ; # [cfg (target_os = "redox")] pub use libc :: { O_EXLOCK , O_SHLOCK , O_SYMLINK } ; pub const MAP_SHARED : u32 = libc :: MAP_SHARED as u32 ; pub const MAP_PRIVATE : u32 = libc :: MAP_PRIVATE as u32 ; pub const PROT_NONE : u32 = libc :: PROT_NONE as u32 ; pub const PROT_READ : u32 = libc :: PROT_READ as u32 ; pub const PROT_WRITE : u32 = libc :: PROT_WRITE as u32 ; pub const PROT_EXEC : u32 = libc :: PROT_EXEC as u32 ; pub const WNOHANG : u32 = libc :: WNOHANG as u32 ; pub const WUNTRACED : u32 = libc :: WUNTRACED as u32 ; pub const WCONTINUED : u32 = libc :: WCONTINUED as u32 ; }
};
}
