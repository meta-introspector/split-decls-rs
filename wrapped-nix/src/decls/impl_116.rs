macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        # [cfg (feature = "signal")] impl FromStr for Signal { type Err = Error ; fn from_str (s : & str) -> Result < Signal > { Ok (match s { "SIGHUP" => Signal :: SIGHUP , "SIGINT" => Signal :: SIGINT , "SIGQUIT" => Signal :: SIGQUIT , "SIGILL" => Signal :: SIGILL , "SIGTRAP" => Signal :: SIGTRAP , "SIGABRT" => Signal :: SIGABRT , "SIGBUS" => Signal :: SIGBUS , "SIGFPE" => Signal :: SIGFPE , "SIGKILL" => Signal :: SIGKILL , "SIGUSR1" => Signal :: SIGUSR1 , "SIGSEGV" => Signal :: SIGSEGV , "SIGUSR2" => Signal :: SIGUSR2 , "SIGPIPE" => Signal :: SIGPIPE , "SIGALRM" => Signal :: SIGALRM , "SIGTERM" => Signal :: SIGTERM , # [cfg (all (any (linux_android , target_os = "emscripten" , target_os = "fuchsia" ,) , not (any (target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "sparc" , target_arch = "sparc64"))))] "SIGSTKFLT" => Signal :: SIGSTKFLT , "SIGCHLD" => Signal :: SIGCHLD , "SIGCONT" => Signal :: SIGCONT , "SIGSTOP" => Signal :: SIGSTOP , "SIGTSTP" => Signal :: SIGTSTP , "SIGTTIN" => Signal :: SIGTTIN , "SIGTTOU" => Signal :: SIGTTOU , "SIGURG" => Signal :: SIGURG , "SIGXCPU" => Signal :: SIGXCPU , "SIGXFSZ" => Signal :: SIGXFSZ , "SIGVTALRM" => Signal :: SIGVTALRM , "SIGPROF" => Signal :: SIGPROF , "SIGWINCH" => Signal :: SIGWINCH , # [cfg (not (target_os = "haiku"))] "SIGIO" => Signal :: SIGIO , # [cfg (any (linux_android , target_os = "emscripten" , target_os = "fuchsia" ,))] "SIGPWR" => Signal :: SIGPWR , "SIGSYS" => Signal :: SIGSYS , # [cfg (not (any (linux_android , target_os = "emscripten" , target_os = "fuchsia" , target_os = "redox" , target_os = "haiku")))] "SIGEMT" => Signal :: SIGEMT , # [cfg (not (any (linux_android , target_os = "emscripten" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "haiku" , target_os = "solaris" , target_os = "cygwin")))] "SIGINFO" => Signal :: SIGINFO , _ => return Err (Errno :: EINVAL) , }) } }
    };
}

impl_116!()