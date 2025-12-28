macro_rules! deps {
    () => {
        ErrnoSentinel!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl ErrnoSentinel for libc :: sighandler_t { fn sentinel () -> Self { libc :: SIG_ERR } }
    };
}

impl_14!();