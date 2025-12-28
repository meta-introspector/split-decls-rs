macro_rules! deps {
    () => {
        NonBindingLetSub!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl Subdiagnostic for NonBindingLetSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let can_suggest_binding = self . drop_fn_start_end . is_some () || ! self . is_assign_desugar ; if can_suggest_binding { let prefix = if self . is_assign_desugar { "let " } else { "" } ; diag . span_suggestion_verbose (self . suggestion , fluent :: lint_non_binding_let_suggestion , format ! ("{prefix}_unused") , Applicability :: MachineApplicable ,) ; } else { diag . span_help (self . suggestion , fluent :: lint_non_binding_let_suggestion) ; } if let Some (drop_fn_start_end) = self . drop_fn_start_end { diag . multipart_suggestion (fluent :: lint_non_binding_let_multi_suggestion , vec ! [(drop_fn_start_end . 0 , "drop(" . to_string ()) , (drop_fn_start_end . 1 , ")" . to_string ()) ,] , Applicability :: MachineApplicable ,) ; } else { diag . help (fluent :: lint_non_binding_let_multi_drop_fn) ; } } }
    };
}

impl_458!();