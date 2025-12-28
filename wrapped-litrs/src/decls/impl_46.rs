macro_rules! deps {
    () => {
        ByteStringLit!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl ByteStringLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> ByteStringLit < String > { ByteStringLit { raw : self . raw . to_owned () , value : self . value , num_hashes : self . num_hashes , start_suffix : self . start_suffix , } } }
    };
}

impl_46!()