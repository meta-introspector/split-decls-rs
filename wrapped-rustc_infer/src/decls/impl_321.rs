macro_rules! deps {
    () => {
        PolyTraitObligation!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn derived_cause (& self , variant : impl FnOnce (DerivedCause < 'tcx >) -> ObligationCauseCode < 'tcx > ,) -> ObligationCause < 'tcx > { self . cause . clone () . derived_cause (self . predicate , variant) } }
    };
}

impl_321!();