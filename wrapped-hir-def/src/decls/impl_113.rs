macro_rules! deps {
    () => {
        TypeOrConstParamId!();
        ConstParamId!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl ConstParamId { # [doc = " Caller should check if this toc id really belongs to a const"] pub fn from_unchecked (it : TypeOrConstParamId) -> Self { Self (it) } }
    };
}

impl_113!()