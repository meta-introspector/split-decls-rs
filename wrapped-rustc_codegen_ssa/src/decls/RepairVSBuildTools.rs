macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! RepairVSBuildTools {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_repair_vs_build_tools)] pub (crate) struct RepairVSBuildTools ;
    };
}

RepairVSBuildTools!()