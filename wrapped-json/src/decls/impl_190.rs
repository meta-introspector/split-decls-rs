macro_rules! deps {
    () => {
        Serializer!();
        CompactFormatter!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < W > Serializer < W > where W : io :: Write , { # [doc = " Creates a new JSON serializer."] # [inline] pub fn new (writer : W) -> Self { Serializer :: with_formatter (writer , CompactFormatter) } }
    };
}

impl_190!();