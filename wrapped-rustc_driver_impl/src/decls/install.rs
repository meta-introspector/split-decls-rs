macro_rules! install {
    () => {
        # [doc = " When one of the KILL signals is delivered to the process, print a stack trace and then exit."] pub (super) fn install () { unsafe { let alt_stack_size : usize = min_sigstack_size () + 64 * 1024 ; let mut alt_stack : libc :: stack_t = mem :: zeroed () ; alt_stack . ss_sp = alloc (Layout :: from_size_align (alt_stack_size , 1) . unwrap ()) . cast () ; alt_stack . ss_size = alt_stack_size ; libc :: sigaltstack (& alt_stack , ptr :: null_mut ()) ; let mut sa : libc :: sigaction = mem :: zeroed () ; sa . sa_sigaction = print_stack_trace as libc :: sighandler_t ; sa . sa_flags = libc :: SA_NODEFER | libc :: SA_RESETHAND | libc :: SA_ONSTACK ; libc :: sigemptyset (& mut sa . sa_mask) ; for (signum , _signame) in KILL_SIGNALS { libc :: sigaction (signum , & sa , ptr :: null_mut ()) ; } } }
    };
}

install!();