macro_rules! deps {
    () => {
        Base64Impl!();
        Base64Variant!();
        Base64UrlSafe!();
        Decoder!();
        Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Decoder for Base64UrlSafe { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: UrlSafe) } }
    };
}

impl_13!();