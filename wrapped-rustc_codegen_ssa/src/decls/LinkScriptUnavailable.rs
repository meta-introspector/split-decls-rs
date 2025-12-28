macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LinkScriptUnavailable {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_link_script_unavailable)] pub (crate) struct LinkScriptUnavailable ;
    };
}

LinkScriptUnavailable!()