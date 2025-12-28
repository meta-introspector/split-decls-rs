macro_rules! deps {
    () => {
        Locatable!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl Locatable for Span { fn to_span (& self , _ : TyCtxt < '_ >) -> Span { * self } }
    };
}

impl_382!()