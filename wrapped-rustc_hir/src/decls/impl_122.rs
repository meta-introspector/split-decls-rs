macro_rules! deps {
    () => {
        ConstArg!();
        Path!();
        ConstArgKind!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'hir , Unambig > ConstArg < 'hir , Unambig > { pub fn anon_const_hir_id (& self) -> Option < HirId > { match self . kind { ConstArgKind :: Anon (ac) => Some (ac . hir_id) , _ => None , } } pub fn span (& self) -> Span { match self . kind { ConstArgKind :: Path (path) => path . span () , ConstArgKind :: Anon (anon) => anon . span , ConstArgKind :: Infer (span , _) => span , } } }
    };
}

impl_122!();