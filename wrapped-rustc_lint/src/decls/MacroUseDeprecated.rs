macro_rules! MacroUseDeprecated {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_macro_use_deprecated)] # [help] pub (crate) struct MacroUseDeprecated ;
    };
}

MacroUseDeprecated!();