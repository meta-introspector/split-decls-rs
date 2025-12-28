macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! AixStripNotUsed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_aix_strip_not_used)] pub (crate) struct AixStripNotUsed ;
    };
}

AixStripNotUsed!()