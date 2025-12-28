macro_rules! deps {
    () => {
        Encoder!();
        Base64UrlSafe!();
        Error!();
        Base64Variant!();
        Base64Impl!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Encoder for Base64UrlSafe { # [inline] fn encoded_len (bin_len : usize) -> Result < usize , Error > { Base64Impl :: encoded_len (bin_len , Base64Variant :: UrlSafe) } # [inline] fn encode < IN : AsRef < [u8] > > (b64 : & mut [u8] , bin : IN) -> Result < & [u8] , Error > { Base64Impl :: encode (b64 , bin . as_ref () , Base64Variant :: UrlSafe) } }
    };
}

impl_12!();