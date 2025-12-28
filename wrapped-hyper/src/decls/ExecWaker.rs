macro_rules! ExecWaker {
    () => {
        struct ExecWaker (AtomicBool) ;
    };
}

ExecWaker!()