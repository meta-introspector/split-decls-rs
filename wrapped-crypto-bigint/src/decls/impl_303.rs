macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl core :: error :: Error for DecodeError { }
    };
}

impl_303!()