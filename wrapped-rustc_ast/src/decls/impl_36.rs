macro_rules! deps {
    () => {
        GenericParam!();
        Lifetime!();
        Type!();
        GenericParamKind!();
        Const!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl GenericParam { pub fn span (& self) -> Span { match & self . kind { GenericParamKind :: Lifetime | GenericParamKind :: Type { default : None } => { self . ident . span } GenericParamKind :: Type { default : Some (ty) } => self . ident . span . to (ty . span) , GenericParamKind :: Const { span , .. } => * span , } } }
    };
}

impl_36!();