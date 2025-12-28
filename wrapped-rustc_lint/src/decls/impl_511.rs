macro_rules! deps {
    () => {
        OverflowingBinHexSign!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl Subdiagnostic for OverflowingBinHexSign { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { OverflowingBinHexSign :: Positive => { diag . note (fluent :: lint_positive_note) ; } OverflowingBinHexSign :: Negative => { diag . note (fluent :: lint_negative_note) ; diag . note (fluent :: lint_negative_becomes_note) ; } } } }
    };
}

impl_511!()