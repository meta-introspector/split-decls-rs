macro_rules! deps {
    () => {
        IntegerLit!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl IntegerLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> IntegerLit < String > { IntegerLit { raw : self . raw . to_owned () , start_main_part : self . start_main_part , end_main_part : self . end_main_part , base : self . base , } } }
    };
}

impl_188!();