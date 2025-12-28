macro_rules! deps {
    () => {
        Generator!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < A : Send , T : Send > Send for Generator < 'static , A , T > { }
    };
}

impl_13!()