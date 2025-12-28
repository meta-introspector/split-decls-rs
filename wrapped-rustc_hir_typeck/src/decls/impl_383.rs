macro_rules! deps {
    () => {
        Locatable!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl Locatable for HirId { fn to_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . hir_span (* self) } }
    };
}

impl_383!();