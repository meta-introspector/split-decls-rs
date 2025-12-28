macro_rules! deps {
    () => {
        Parameter!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl From < ty :: EarlyParamRegion > for Parameter { fn from (param : ty :: EarlyParamRegion) -> Self { Parameter (param . index) } }
    };
}

impl_254!()