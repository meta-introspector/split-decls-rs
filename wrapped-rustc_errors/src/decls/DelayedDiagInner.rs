macro_rules! DelayedDiagInner {
    () => {
        struct DelayedDiagInner { inner : DiagInner , note : Backtrace , }
    };
}

DelayedDiagInner!()