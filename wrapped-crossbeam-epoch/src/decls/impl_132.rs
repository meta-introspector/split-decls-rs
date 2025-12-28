macro_rules! deps {
    () => {
        OnceLock!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for OnceLock < T > { }
    };
}

impl_132!()