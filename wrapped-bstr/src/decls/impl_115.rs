macro_rules! deps {
    () => {
        ByteVec!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl ByteVec for Vec < u8 > { # [inline] fn as_vec (& self) -> & Vec < u8 > { self } # [inline] fn as_vec_mut (& mut self) -> & mut Vec < u8 > { self } # [inline] fn into_vec (self) -> Vec < u8 > { self } }
    };
}

impl_115!()