macro_rules! deps {
    () => {
        Config!();
        Writer!();
        EncoderImpl!();
        Encoder!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl < W : Writer , C : Config > EncoderImpl < W , C > { # [doc = " Create a new Encoder"] pub const fn new (writer : W , config : C) -> EncoderImpl < W , C > { EncoderImpl { writer , config } } # [doc = " Return the underlying writer"] # [inline] pub fn into_writer (self) -> W { self . writer } }
    };
}

impl_412!();