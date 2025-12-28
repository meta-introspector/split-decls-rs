macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] impl From < getrandom :: Error > for UnknownCryptoError { fn from (_ : getrandom :: Error) -> Self { UnknownCryptoError } }
    };
}

impl_67!()