macro_rules! DefaultHashTypesDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_default_hash_types)] # [note] pub (crate) struct DefaultHashTypesDiag < 'a > { pub preferred : & 'a str , pub used : Symbol , }
    };
}

DefaultHashTypesDiag!()