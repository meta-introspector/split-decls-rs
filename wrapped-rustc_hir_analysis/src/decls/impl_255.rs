macro_rules! deps {
    () => {
        Parameter!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl From < ty :: ParamConst > for Parameter { fn from (param : ty :: ParamConst) -> Self { Parameter (param . index) } }
    };
}

impl_255!();