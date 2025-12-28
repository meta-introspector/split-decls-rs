macro_rules! deps {
    () => {
        Error!();
        Base64Variant!();
        Encoder!();
        Base64NoPadding!();
        Base64Impl!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Encoder for Base64NoPadding { # [inline] fn encoded_len (bin_len : usize) -> Result < usize , Error > { Base64Impl :: encoded_len (bin_len , Base64Variant :: OriginalNoPadding) } # [inline] fn encode < IN : AsRef < [u8] > > (b64 : & mut [u8] , bin : IN) -> Result < & [u8] , Error > { Base64Impl :: encode (b64 , bin . as_ref () , Base64Variant :: OriginalNoPadding) } }
    };
}

impl_10!();