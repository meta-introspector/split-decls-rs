macro_rules! deps {
    () => {
        CodegenLintLevels!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        impl CodegenLintLevels { pub fn from_tcx (tcx : TyCtxt < '_ >) -> Self { Self { linker_messages : tcx . lint_level_at_node (LINKER_MESSAGES , CRATE_HIR_ID) } } }
    };
}

impl_615!();