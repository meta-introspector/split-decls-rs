macro_rules! deps {
    () => {
        Parameter!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl From < ty :: ParamTy > for Parameter { fn from (param : ty :: ParamTy) -> Self { Parameter (param . index) } }
    };
}

impl_253!();