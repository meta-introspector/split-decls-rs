macro_rules! deps {
    () => {
        GenericHkdfExtract!();
    };
}

macro_rules! SimpleHkdfExtract {
    () => {
        deps!();
        # [doc = " [`GenericHkdfExtract`] variant which uses [`SimpleHmac`] for the underlying HMAC implementation."] pub type SimpleHkdfExtract < H > = GenericHkdfExtract < SimpleHmac < H > > ;
    };
}

SimpleHkdfExtract!();