macro_rules! deps {
    () => {
        Base64Variant!();
        Base64Impl!();
        Encoder!();
        Base64UrlSafeNoPadding!();
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Encoder for Base64UrlSafeNoPadding { # [inline] fn encoded_len (bin_len : usize) -> Result < usize , Error > { Base64Impl :: encoded_len (bin_len , Base64Variant :: UrlSafeNoPadding) } # [inline] fn encode < IN : AsRef < [u8] > > (b64 : & mut [u8] , bin : IN) -> Result < & [u8] , Error > { Base64Impl :: encode (b64 , bin . as_ref () , Base64Variant :: UrlSafeNoPadding) } }
    };
}

impl_14!();