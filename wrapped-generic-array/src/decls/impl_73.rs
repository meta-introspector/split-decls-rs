macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T : ConstDefault , U : ArrayLength > GenericArray < T , U > where Self : ConstDefault , { # [doc = " Returns the constant \"default value\" for an array using [ConstDefault]"] # [inline (always)] pub const fn const_default () -> Self { Self :: DEFAULT } }
    };
}

impl_73!()