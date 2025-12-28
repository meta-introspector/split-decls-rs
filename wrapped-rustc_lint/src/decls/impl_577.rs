macro_rules! deps {
    () => {
        UnstableFeature!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for UnstableFeature { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (self . msg) ; } }
    };
}

impl_577!();