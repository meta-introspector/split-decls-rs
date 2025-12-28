macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! XcrunError {
    () => {
        deps!();
        # [derive (Diagnostic , Debug)] pub (crate) enum XcrunError { # [diag (codegen_ssa_xcrun_failed_invoking)] FailedInvoking { sdk_name : & 'static str , command_formatted : String , error : std :: io :: Error } , # [diag (codegen_ssa_xcrun_unsuccessful)] # [note] Unsuccessful { sdk_name : & 'static str , command_formatted : String , stdout : String , stderr : String , } , }
    };
}

XcrunError!();