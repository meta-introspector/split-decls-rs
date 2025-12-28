macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LinkerUnsupportedModifier {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_linker_unsupported_modifier)] pub (crate) struct LinkerUnsupportedModifier ;
    };
}

LinkerUnsupportedModifier!();