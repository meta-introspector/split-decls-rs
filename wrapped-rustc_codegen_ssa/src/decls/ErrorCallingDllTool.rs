macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ErrorCallingDllTool {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_error_calling_dlltool)] pub (crate) struct ErrorCallingDllTool < 'a > { pub dlltool_path : Cow < 'a , str > , pub error : std :: io :: Error , }
    };
}

ErrorCallingDllTool!()