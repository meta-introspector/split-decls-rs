macro_rules! UnusedMacroDefinition {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_macro_definition)] pub (crate) struct UnusedMacroDefinition { pub name : Symbol , }
    };
}

UnusedMacroDefinition!()