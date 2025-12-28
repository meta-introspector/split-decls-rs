macro_rules! deps {
    () => {
        Config!();
        Writer!();
        Sealed!();
        EncoderImpl!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl < W : Writer , C : Config > Sealed for EncoderImpl < W , C > { }
    };
}

impl_414!()