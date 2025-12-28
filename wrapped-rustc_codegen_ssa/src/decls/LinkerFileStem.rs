macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LinkerFileStem {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_linker_file_stem)] pub (crate) struct LinkerFileStem ;
    };
}

LinkerFileStem!();