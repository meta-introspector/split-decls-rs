macro_rules! deps {
    () => {
        TypeOrConstParamId!();
        TypeParamId!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl From < TypeParamId > for TypeOrConstParamId { fn from (it : TypeParamId) -> Self { it . 0 } }
    };
}

impl_110!()