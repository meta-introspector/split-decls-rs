macro_rules! deps {
    () => {
        UnusedDef!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for UnusedDef < '_ , '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_unused_def) ; diag . arg ("pre" , self . pre) ; diag . arg ("post" , self . post) ; diag . arg ("def" , self . cx . tcx . def_path_str (self . def_id)) ; if let Some (note) = self . note { diag . note (note . to_string ()) ; } if let Some (sugg) = self . suggestion { diag . subdiagnostic (sugg) ; } } }
    };
}

impl_546!()