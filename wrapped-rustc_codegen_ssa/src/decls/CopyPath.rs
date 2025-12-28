macro_rules! deps {
    () => {
        Diagnostic!();
        DebugArgPath!();
    };
}

macro_rules! CopyPath {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_copy_path)] pub struct CopyPath < 'a > { from : DebugArgPath < 'a > , to : DebugArgPath < 'a > , error : Error , }
    };
}

CopyPath!()