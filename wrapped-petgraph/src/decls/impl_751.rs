macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
    };
}

macro_rules! impl_751 {
    () => {
        deps!();
        impl < 'a , Ix , E > EdgeReference < 'a , E , Ix > where Ix : IndexType , { # [doc = " Access the edge’s weight."] # [doc = ""] # [doc = " **NOTE** that this method offers a longer lifetime"] # [doc = " than the trait (unfortunately they don't match yet)."] pub fn weight (& self) -> & 'a E { self . weight } }
    };
}

impl_751!()