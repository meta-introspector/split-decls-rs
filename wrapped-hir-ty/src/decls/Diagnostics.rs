macro_rules! Diagnostics {
    () => {
        pub (crate) type Diagnostics = Option < ThinArc < () , TyLoweringDiagnostic > > ;
    };
}

Diagnostics!();