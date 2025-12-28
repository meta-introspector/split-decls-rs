macro_rules! DiagOutOfImpl {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_diag_out_of_impl)] pub (crate) struct DiagOutOfImpl ;
    };
}

DiagOutOfImpl!()