macro_rules! UnknownMacroVariable {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unknown_macro_variable)] pub (crate) struct UnknownMacroVariable { pub name : MacroRulesNormalizedIdent , }
    };
}

UnknownMacroVariable!()