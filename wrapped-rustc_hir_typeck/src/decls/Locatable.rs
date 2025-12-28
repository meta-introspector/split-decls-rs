macro_rules! Locatable {
    () => {
        pub (crate) trait Locatable { fn to_span (& self , tcx : TyCtxt < '_ >) -> Span ; }
    };
}

Locatable!();