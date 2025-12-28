macro_rules! VarNeedNotMut {
    () => {
        # [derive (LintDiagnostic)] # [diag (borrowck_var_does_not_need_mut)] pub (crate) struct VarNeedNotMut { # [suggestion (style = "short" , applicability = "machine-applicable" , code = "")] pub span : Span , }
    };
}

VarNeedNotMut!();