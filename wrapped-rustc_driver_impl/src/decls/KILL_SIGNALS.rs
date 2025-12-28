macro_rules! KILL_SIGNALS {
    () => {
        # [doc = " Signals that represent that we have a bug, and our prompt termination has"] # [doc = " been ordered."] # [rustfmt :: skip] const KILL_SIGNALS : [(libc :: c_int , & str) ; 3] = [(libc :: SIGILL , "SIGILL") , (libc :: SIGBUS , "SIGBUS") , (libc :: SIGSEGV , "SIGSEGV")] ;
    };
}

KILL_SIGNALS!();