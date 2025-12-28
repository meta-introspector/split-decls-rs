macro_rules! deps {
    () => {
        Ty!();
        FnRetTy!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl FnRetTy { pub fn span (& self) -> Span { match self { & FnRetTy :: Default (span) => span , FnRetTy :: Ty (ty) => ty . span , } } }
    };
}

impl_178!()