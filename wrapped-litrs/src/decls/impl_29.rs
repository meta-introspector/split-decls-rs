macro_rules! deps {
    () => {
        ByteLit!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl ByteLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> ByteLit < String > { ByteLit { raw : self . raw . to_owned () , start_suffix : self . start_suffix , value : self . value , } } }
    };
}

impl_29!();