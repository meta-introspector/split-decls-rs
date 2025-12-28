macro_rules! deps {
    () => {
        EdgeType!();
        EdgeReference!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl < 'a , Ty , E , Ix > EdgeReference < 'a , E , Ty , Ix > where Ty : EdgeType , { # [doc = " Access the edge’s weight."] # [doc = ""] # [doc = " **NOTE** that this method offers a longer lifetime"] # [doc = " than the trait (unfortunately they don't match yet)."] pub fn weight (& self) -> & 'a E { self . weight } }
    };
}

impl_526!();