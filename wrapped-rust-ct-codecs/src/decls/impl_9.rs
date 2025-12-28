macro_rules! deps {
    () => {
        Base64Variant!();
        Error!();
        Decoder!();
        Base64Impl!();
        Base64!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Decoder for Base64 { # [inline] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , b64 : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { Base64Impl :: decode (bin , b64 . as_ref () , ignore , Base64Variant :: Original) } }
    };
}

impl_9!();