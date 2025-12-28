macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (feature = "zerovec")] unsafe impl zerovec :: ule :: VarULE for PotentialUtf8 { # [inline] fn validate_bytes (_ : & [u8]) -> Result < () , zerovec :: ule :: UleError > { Ok (()) } # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { PotentialUtf8 :: from_bytes (bytes) } }
    };
}

impl_32!();