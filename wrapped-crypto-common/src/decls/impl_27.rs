macro_rules! deps {
    () => {
        KeySizeUser!();
        InnerUser!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T > KeySizeUser for T where T : InnerUser , T :: Inner : KeySizeUser , { type KeySize = < T :: Inner as KeySizeUser > :: KeySize ; }
    };
}

impl_27!()