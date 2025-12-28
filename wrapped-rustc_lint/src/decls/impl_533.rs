macro_rules! deps {
    () => {
        ImproperCTypes!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for ImproperCTypes < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_improper_ctypes) ; diag . arg ("ty" , self . ty) ; diag . arg ("desc" , self . desc) ; diag . span_label (self . label , fluent :: lint_label) ; if let Some (help) = self . help { diag . help (help) ; } diag . note (self . note) ; if let Some (note) = self . span_note { diag . span_note (note , fluent :: lint_note) ; } } }
    };
}

impl_533!()