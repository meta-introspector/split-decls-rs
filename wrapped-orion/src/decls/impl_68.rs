macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] impl From < ct_codecs :: Error > for UnknownCryptoError { fn from (_ : ct_codecs :: Error) -> Self { UnknownCryptoError } }
    };
}

impl_68!();