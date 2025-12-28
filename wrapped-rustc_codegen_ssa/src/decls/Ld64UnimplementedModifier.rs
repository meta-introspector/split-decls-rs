macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! Ld64UnimplementedModifier {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_ld64_unimplemented_modifier)] pub (crate) struct Ld64UnimplementedModifier ;
    };
}

Ld64UnimplementedModifier!();