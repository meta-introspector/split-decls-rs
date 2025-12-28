macro_rules! deps {
    () => {
        SampleUniform!();
        SampleBorrow!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < Borrowed > SampleBorrow < Borrowed > for & Borrowed where Borrowed : SampleUniform , { # [inline (always)] fn borrow (& self) -> & Borrowed { self } }
    };
}

impl_171!()