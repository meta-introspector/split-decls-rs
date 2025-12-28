macro_rules! deps {
    () => {
        AmbiguousGlobImports!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > LintDiagnostic < 'a , G > for AmbiguousGlobImports { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , G >) { diag . primary_message (self . ambiguity . msg . clone ()) ; rustc_errors :: report_ambiguity_error (diag , self . ambiguity) ; } }
    };
}

impl_628!();