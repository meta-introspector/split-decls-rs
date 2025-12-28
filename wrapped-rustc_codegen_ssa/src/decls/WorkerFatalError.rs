macro_rules! WorkerFatalError {
    () => {
        # [doc = " `FatalError` is explicitly not `Send`."] # [must_use] pub (crate) struct WorkerFatalError ;
    };
}

WorkerFatalError!();