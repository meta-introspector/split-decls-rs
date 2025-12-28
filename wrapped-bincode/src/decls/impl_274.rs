macro_rules! deps {
    () => {
        Sealed!();
        DecoderImpl!();
        Config!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < R , C : Config , Context > Sealed for DecoderImpl < R , C , Context > { }
    };
}

impl_274!()