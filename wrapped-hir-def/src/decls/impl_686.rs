macro_rules! deps {
    () => {
        TypeOrConstParamId!();
        ConstParamId!();
    };
}

macro_rules! impl_686 {
    () => {
        deps!();
        impl From < ConstParamId > for TypeOrConstParamId { fn from (it : ConstParamId) -> Self { it . 0 } }
    };
}

impl_686!()