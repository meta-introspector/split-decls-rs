macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! XcrunSdkPathWarning {
    () => {
        deps!();
        # [derive (Diagnostic , Debug)] # [diag (codegen_ssa_xcrun_sdk_path_warning)] # [note] pub (crate) struct XcrunSdkPathWarning { pub sdk_name : & 'static str , pub stderr : String , }
    };
}

XcrunSdkPathWarning!()