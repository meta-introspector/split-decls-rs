macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Span { # [doc = " A no-op"] pub fn record < V > (& self , _field : & str , _value : V) -> & Self { self } }
    };
}

impl_16!()