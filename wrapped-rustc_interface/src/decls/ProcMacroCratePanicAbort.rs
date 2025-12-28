macro_rules! ProcMacroCratePanicAbort {
    () => {
        # [derive (Diagnostic)] # [diag (interface_proc_macro_crate_panic_abort)] pub struct ProcMacroCratePanicAbort ;
    };
}

ProcMacroCratePanicAbort!();