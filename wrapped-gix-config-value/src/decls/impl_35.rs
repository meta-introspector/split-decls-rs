macro_rules! deps {
    () => {
        Suffix!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Suffix { # [doc = " Returns the number of bits that the suffix shifts left by."] # [must_use] pub const fn bitwise_offset (self) -> usize { match self { Self :: Kibi => 10 , Self :: Mebi => 20 , Self :: Gibi => 30 , } } }
    };
}

impl_35!()