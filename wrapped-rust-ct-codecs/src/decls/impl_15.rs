macro_rules! deps {
    () => {
        Base64UrlSafeNoPadding!();
        Error!();
        Base64Impl!();
        Decoder!();
        Base64Variant!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Decoder for Base64UrlSafeNoPadding { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: UrlSafeNoPadding) } }
    };
}

impl_15!();