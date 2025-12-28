macro_rules! deps {
    () => {
        Lcg64Xsh32!();
    };
}

macro_rules! Pcg32 {
    () => {
        deps!();
        # [doc = " [`Lcg64Xsh32`] is also officially known as `pcg32`."] pub type Pcg32 = Lcg64Xsh32 ;
    };
}

Pcg32!()