macro_rules! deps {
    () => {
        KeySizeUser!();
    };
}

macro_rules! Key {
    () => {
        deps!();
        # [doc = " Key used by [`KeySizeUser`] implementors."] pub type Key < B > = Array < u8 , < B as KeySizeUser > :: KeySize > ;
    };
}

Key!();