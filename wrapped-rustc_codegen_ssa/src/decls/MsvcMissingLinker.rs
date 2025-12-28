macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MsvcMissingLinker {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_msvc_missing_linker)] pub (crate) struct MsvcMissingLinker ;
    };
}

MsvcMissingLinker!();