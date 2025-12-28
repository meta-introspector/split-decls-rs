macro_rules! deps {
    () => {
        Lifetime!();
        GenericArg!();
        Const!();
        Type!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl GenericArg { pub fn span (& self) -> Span { match self { GenericArg :: Lifetime (lt) => lt . ident . span , GenericArg :: Type (ty) => ty . span , GenericArg :: Const (ct) => ct . value . span , } } }
    };
}

impl_19!();