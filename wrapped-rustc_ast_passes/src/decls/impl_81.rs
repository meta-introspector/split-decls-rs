macro_rules! deps {
    () => {
        StableFeature!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Subdiagnostic for StableFeature { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("name" , self . name) ; diag . arg ("since" , self . since) ; diag . help (fluent :: ast_passes_stable_since) ; } }
    };
}

impl_81!()