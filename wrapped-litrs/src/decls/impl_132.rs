macro_rules! deps {
    () => {
        FloatLit!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl FloatLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> FloatLit < String > { FloatLit { raw : self . raw . to_owned () , end_integer_part : self . end_integer_part , end_fractional_part : self . end_fractional_part , end_number_part : self . end_number_part , } } }
    };
}

impl_132!()