macro_rules! deps {
    () => {
        ActualSamplingMode!();
        SamplingMethod!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl From < crate :: ActualSamplingMode > for SamplingMethod { fn from (other : crate :: ActualSamplingMode) -> Self { match other { crate :: ActualSamplingMode :: Flat => SamplingMethod :: Flat , crate :: ActualSamplingMode :: Linear => SamplingMethod :: Linear , } } }
    };
}

impl_77!()