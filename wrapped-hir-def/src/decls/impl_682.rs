macro_rules! deps {
    () => {
        TypeOrConstParamId!();
        TypeParamId!();
    };
}

macro_rules! impl_682 {
    () => {
        deps!();
        impl From < TypeParamId > for TypeOrConstParamId { fn from (it : TypeParamId) -> Self { it . 0 } }
    };
}

impl_682!()