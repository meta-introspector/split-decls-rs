macro_rules! other_51 {
    () => {
        unsafe extern "C" { fn backtrace_symbols_fd (buffer : * const * mut libc :: c_void , size : libc :: c_int , fd : libc :: c_int) ; }
    };
}

other_51!();