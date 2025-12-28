macro_rules! deps {
    () => {
        Base64NoPadding!();
        Error!();
        Base64Impl!();
        Decoder!();
        Base64Variant!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Decoder for Base64NoPadding { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: OriginalNoPadding) } }
    };
}

impl_11!()