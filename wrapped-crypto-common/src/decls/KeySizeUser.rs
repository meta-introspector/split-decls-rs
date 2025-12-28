macro_rules! deps {
    () => {
        KeyIvInit!();
        KeyInit!();
        Key!();
    };
}

macro_rules! KeySizeUser {
    () => {
        deps!();
        # [doc = " Types which use key for initialization."] # [doc = ""] # [doc = " Generally it's used indirectly via [`KeyInit`] or [`KeyIvInit`]."] pub trait KeySizeUser { # [doc = " Key size in bytes."] type KeySize : ArraySize ; # [doc = " Return key size in bytes."] # [inline (always)] fn key_size () -> usize { Self :: KeySize :: USIZE } }
    };
}

KeySizeUser!()