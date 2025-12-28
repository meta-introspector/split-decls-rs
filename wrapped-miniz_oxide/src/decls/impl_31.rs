macro_rules! deps {
    () => {
        CompressionStrategy!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl From < CompressionStrategy > for i32 { # [inline (always)] fn from (value : CompressionStrategy) -> Self { value as i32 } }
    };
}

impl_31!()