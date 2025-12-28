macro_rules! deps {
    () => {
        TypeParamId!();
        TypeOrConstParamId!();
    };
}

macro_rules! impl_681 {
    () => {
        deps!();
        impl TypeParamId { # [doc = " Caller should check if this toc id really belongs to a type"] pub fn from_unchecked (it : TypeOrConstParamId) -> Self { Self (it) } }
    };
}

impl_681!()