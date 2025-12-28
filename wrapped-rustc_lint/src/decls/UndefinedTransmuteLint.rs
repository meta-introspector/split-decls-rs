macro_rules! UndefinedTransmuteLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_undefined_transmute)] # [note] # [note (lint_note2)] # [help] pub (crate) struct UndefinedTransmuteLint ;
    };
}

UndefinedTransmuteLint!();