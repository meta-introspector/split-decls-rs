macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        unsafe impl < T > Send for Buffer < T > { }
    };
}

impl_5!()