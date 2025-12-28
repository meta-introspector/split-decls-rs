macro_rules! ThreadLocalMode {
    () => {
        # [doc = " LLVMThreadLocalMode"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum ThreadLocalMode { # [expect (dead_code)] NotThreadLocal , GeneralDynamic , LocalDynamic , InitialExec , LocalExec , }
    };
}

ThreadLocalMode!();