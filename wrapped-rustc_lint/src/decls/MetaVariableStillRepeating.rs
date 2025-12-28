macro_rules! MetaVariableStillRepeating {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_metavariable_still_repeating)] pub (crate) struct MetaVariableStillRepeating { pub name : MacroRulesNormalizedIdent , }
    };
}

MetaVariableStillRepeating!()