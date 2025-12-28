macro_rules! deps {
    () => {
        PolyTraitObligation!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn polarity (& self) -> ty :: PredicatePolarity { self . predicate . skip_binder () . polarity } pub fn self_ty (& self) -> ty :: Binder < 'tcx , Ty < 'tcx > > { self . predicate . map_bound (| p | p . self_ty ()) } }
    };
}

impl_326!()