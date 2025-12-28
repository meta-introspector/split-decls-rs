macro_rules! MetaVariableWrongOperator {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_metavariable_wrong_operator)] pub (crate) struct MetaVariableWrongOperator ;
    };
}

MetaVariableWrongOperator!();