macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! NoNatvisDirectory {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_no_natvis_directory)] pub (crate) struct NoNatvisDirectory { pub error : Error , }
    };
}

NoNatvisDirectory!()