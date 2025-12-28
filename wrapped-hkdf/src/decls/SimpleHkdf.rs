macro_rules! deps {
    () => {
        GenericHkdf!();
    };
}

macro_rules! SimpleHkdf {
    () => {
        deps!();
        # [doc = " [`GenericHkdf`] variant which uses [`SimpleHmac`] for the underlying HMAC implementation."] pub type SimpleHkdf < H > = GenericHkdf < SimpleHmac < H > > ;
    };
}

SimpleHkdf!()