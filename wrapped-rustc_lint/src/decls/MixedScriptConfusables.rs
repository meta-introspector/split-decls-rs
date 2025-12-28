macro_rules! MixedScriptConfusables {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_mixed_script_confusables)] # [note (lint_includes_note)] # [note] pub (crate) struct MixedScriptConfusables { pub set : String , pub includes : String , }
    };
}

MixedScriptConfusables!();