macro_rules! deps {
    () => {
        Writer!();
        Encoder!();
        EncoderImpl!();
        Config!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < W : Writer , C : Config > Encoder for EncoderImpl < W , C > { type W = W ; type C = C ; # [inline] fn writer (& mut self) -> & mut Self :: W { & mut self . writer } # [inline] fn config (& self) -> & Self :: C { & self . config } }
    };
}

impl_413!();