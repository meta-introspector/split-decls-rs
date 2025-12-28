macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SelfContainedLinkerMissing {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_self_contained_linker_missing)] pub (crate) struct SelfContainedLinkerMissing ;
    };
}

SelfContainedLinkerMissing!();