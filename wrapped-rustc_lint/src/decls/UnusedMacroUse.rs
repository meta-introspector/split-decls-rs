macro_rules! UnusedMacroUse {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_macro_use)] pub (crate) struct UnusedMacroUse ;
    };
}

UnusedMacroUse!();