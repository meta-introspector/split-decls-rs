macro_rules! deps {
    () => {
        RedundantImportSub!();
    };
}

macro_rules! RedundantImport {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_redundant_import)] pub (crate) struct RedundantImport { # [subdiagnostic] pub subs : Vec < RedundantImportSub > , pub ident : Ident , }
    };
}

RedundantImport!();