macro_rules! UnknownCryptoError {
    () => {
        # [allow (clippy :: derive_partial_eq_without_eq)] # [doc = " Opaque error."] # [derive (Clone , Copy , PartialEq)] pub struct UnknownCryptoError ;
    };
}

UnknownCryptoError!()