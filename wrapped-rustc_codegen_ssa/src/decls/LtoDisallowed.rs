macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LtoDisallowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_lto_disallowed)] pub (crate) struct LtoDisallowed ;
    };
}

LtoDisallowed!()