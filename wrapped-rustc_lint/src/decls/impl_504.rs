macro_rules! deps {
    () => {
        DropGlue!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for DropGlue < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_drop_glue) ; diag . arg ("needs_drop" , self . tcx . def_path_str (self . def_id)) ; } }
    };
}

impl_504!()