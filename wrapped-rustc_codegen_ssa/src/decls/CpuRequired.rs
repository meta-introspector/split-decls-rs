macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CpuRequired {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_cpu_required)] pub (crate) struct CpuRequired ;
    };
}

CpuRequired!();