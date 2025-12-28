macro_rules! deps {
    () => {
        GenericHkdfExtract!();
    };
}

macro_rules! HkdfExtract {
    () => {
        deps!();
        # [doc = " [`GenericHkdfExtract`] variant which uses [`Hmac`] for the underlying HMAC implementation."] pub type HkdfExtract < H > = GenericHkdfExtract < Hmac < H > > ;
    };
}

HkdfExtract!();