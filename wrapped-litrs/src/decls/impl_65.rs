macro_rules! deps {
    () => {
        CharLit!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl CharLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> CharLit < String > { CharLit { raw : self . raw . to_owned () , start_suffix : self . start_suffix , value : self . value , } } }
    };
}

impl_65!();