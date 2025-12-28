macro_rules! DiagnosticLevel {
    () => {
        # [doc = " LLVMRustDiagnosticLevel"] # [derive (Copy , Clone)] # [repr (C)] # [allow (dead_code)] pub (crate) enum DiagnosticLevel { Error , Warning , Note , Remark , }
    };
}

DiagnosticLevel!();