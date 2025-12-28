macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl core :: error :: Error for UnknownCryptoError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { None } }
    };
}

impl_66!()