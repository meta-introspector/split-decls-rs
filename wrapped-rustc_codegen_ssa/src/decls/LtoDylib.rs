macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LtoDylib {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_lto_dylib)] pub (crate) struct LtoDylib ;
    };
}

LtoDylib!()