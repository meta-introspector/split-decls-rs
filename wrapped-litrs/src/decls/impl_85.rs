macro_rules! deps {
    () => {
        CStringLit!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl CStringLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> CStringLit < String > { CStringLit { raw : self . raw . to_owned () , value : self . value , num_hashes : self . num_hashes , start_suffix : self . start_suffix , } } }
    };
}

impl_85!()