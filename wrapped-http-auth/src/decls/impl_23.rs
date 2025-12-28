macro_rules! deps {
    () => {
        BasicClient!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl BasicClient { pub fn realm (& self) -> & str { & self . realm } # [doc = " Responds to the challenge with the supplied parameters."] # [doc = ""] # [doc = " This is functionally identical to [`encode_credentials`]; no parameters"] # [doc = " of the `BasicClient` are needed to produce the credentials."] # [inline] pub fn respond (& self , username : & str , password : & str) -> String { encode_credentials (username , password) } }
    };
}

impl_23!()