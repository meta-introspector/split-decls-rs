macro_rules! deps {
    () => {
        TypeOrConstParamId!();
        TypeParamId!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl TypeParamId { # [doc = " Caller should check if this toc id really belongs to a type"] pub fn from_unchecked (it : TypeOrConstParamId) -> Self { Self (it) } }
    };
}

impl_109!()