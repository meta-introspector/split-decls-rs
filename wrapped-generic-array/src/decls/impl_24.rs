macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : Default , N : ArrayLength > Default for GenericArray < T , N > { # [inline (always)] fn default () -> Self { Self :: generate (| _ | T :: default ()) } }
    };
}

impl_24!()