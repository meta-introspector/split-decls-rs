macro_rules! PathStatementNoEffect {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_path_statement_no_effect)] pub (crate) struct PathStatementNoEffect ;
    };
}

PathStatementNoEffect!()