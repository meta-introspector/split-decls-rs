macro_rules! deps {
    () => {
        ConstParamId!();
        TypeOrConstParamId!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl From < ConstParamId > for TypeOrConstParamId { fn from (it : ConstParamId) -> Self { it . 0 } }
    };
}

impl_114!()