macro_rules! deps {
    () => {
        Section!();
        Key!();
        Checkout!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl Section for Checkout { fn name (& self) -> & str { "checkout" } fn keys (& self) -> & [& dyn Key] { & [& Self :: WORKERS] } }
    };
}

impl_595!()