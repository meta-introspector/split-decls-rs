macro_rules! deps {
    () => {
        Frozen!();
        Create!();
    };
}

macro_rules! impl_758 {
    () => {
        deps!();
        impl < 'a , G > Frozen < 'a , G > { # [doc = " Create a new `Frozen` from a mutable reference to a graph."] pub fn new (gr : & 'a mut G) -> Self { Frozen (gr) } }
    };
}

impl_758!()