macro_rules! deps {
    () => {
        DeserializeStateError!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl core :: error :: Error for DeserializeStateError { }
    };
}

impl_5!()