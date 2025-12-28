macro_rules! deps {
    () => {
        GenericHkdf!();
    };
}

macro_rules! Hkdf {
    () => {
        deps!();
        # [doc = " [`GenericHkdf`] variant which uses [`Hmac`] for the underlying HMAC implementation."] pub type Hkdf < H > = GenericHkdf < Hmac < H > > ;
    };
}

Hkdf!()