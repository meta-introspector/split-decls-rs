macro_rules! deps {
    () => {
        EncoderImpl!();
        Config!();
        Sealed!();
        Writer!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl < W : Writer , C : Config > Sealed for EncoderImpl < W , C > { }
    };
}

impl_414!();