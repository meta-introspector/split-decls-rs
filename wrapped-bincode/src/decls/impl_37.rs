macro_rules! deps {
    () => {
        VecWriter!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl VecWriter { # [doc = " Create a new vec writer with the given capacity"] pub fn with_capacity (cap : usize) -> Self { Self { inner : Vec :: with_capacity (cap) , } } # [allow (dead_code)] pub (crate) fn collect (self) -> Vec < u8 > { self . inner } }
    };
}

impl_37!()