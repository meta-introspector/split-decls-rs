macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Queue < T > { }
    };
}

impl_139!()