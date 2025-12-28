macro_rules! deps {
    () => {
        ForeignItemKind!();
        TyAlias!();
        MacCall!();
        Fn!();
        StaticItem!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl ForeignItemKind { pub fn ident (& self) -> Option < Ident > { match * self { ForeignItemKind :: Static (box StaticItem { ident , .. }) | ForeignItemKind :: Fn (box Fn { ident , .. }) | ForeignItemKind :: TyAlias (box TyAlias { ident , .. }) => Some (ident) , ForeignItemKind :: MacCall (_) => None , } } }
    };
}

impl_234!()