macro_rules! deps {
    () => {
        Engine!();
        EncoderStringWriter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'e , E : Engine > EncoderStringWriter < 'e , E , String > { # [doc = " Create a `EncoderStringWriter` that will encode into a new `String` with the provided config."] pub fn new (engine : & 'e E) -> Self { EncoderStringWriter :: from_consumer (String :: new () , engine) } }
    };
}

impl_51!();