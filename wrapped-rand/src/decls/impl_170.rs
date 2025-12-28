macro_rules! deps {
    () => {
        SampleBorrow!();
        SampleUniform!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < Borrowed > SampleBorrow < Borrowed > for Borrowed where Borrowed : SampleUniform , { # [inline (always)] fn borrow (& self) -> & Borrowed { self } }
    };
}

impl_170!();