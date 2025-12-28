macro_rules! deps {
    () => {
        Formatter!();
        Serializer!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < W , F > Serializer < W , F > where W : io :: Write , F : Formatter , { # [doc = " Creates a new JSON visitor whose output will be written to the writer"] # [doc = " specified."] # [inline] pub fn with_formatter (writer : W , formatter : F) -> Self { Serializer { writer , formatter } } # [doc = " Unwrap the `Writer` from the `Serializer`."] # [inline] pub fn into_inner (self) -> W { self . writer } }
    };
}

impl_192!()