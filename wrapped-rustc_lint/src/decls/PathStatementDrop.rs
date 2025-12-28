macro_rules! deps {
    () => {
        PathStatementDropSub!();
    };
}

macro_rules! PathStatementDrop {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_path_statement_drop)] pub (crate) struct PathStatementDrop { # [subdiagnostic] pub sub : PathStatementDropSub , }
    };
}

PathStatementDrop!();