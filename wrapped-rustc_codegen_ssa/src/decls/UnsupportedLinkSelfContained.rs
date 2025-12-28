macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnsupportedLinkSelfContained {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unsupported_link_self_contained)] pub (crate) struct UnsupportedLinkSelfContained ;
    };
}

UnsupportedLinkSelfContained!()