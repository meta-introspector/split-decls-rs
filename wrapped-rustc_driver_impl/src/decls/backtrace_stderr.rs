macro_rules! backtrace_stderr {
    () => {
        fn backtrace_stderr (buffer : & [* mut libc :: c_void]) { let size = buffer . len () . try_into () . unwrap_or_default () ; unsafe { backtrace_symbols_fd (buffer . as_ptr () , size , libc :: STDERR_FILENO) } ; }
    };
}

backtrace_stderr!();