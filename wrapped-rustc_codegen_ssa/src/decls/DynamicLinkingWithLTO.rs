macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! DynamicLinkingWithLTO {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_dynamic_linking_with_lto)] # [note] pub (crate) struct DynamicLinkingWithLTO ;
    };
}

DynamicLinkingWithLTO!()