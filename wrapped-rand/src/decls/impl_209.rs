macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < R : RngCore + ? Sized > Rng for R { }
    };
}

impl_209!()