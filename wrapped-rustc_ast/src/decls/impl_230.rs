macro_rules! deps {
    () => {
        Defaultness!();
        AssocItemKind!();
        ConstItem!();
        TyAlias!();
        DelegationMac!();
        Const!();
        MacCall!();
        Fn!();
        Type!();
        Delegation!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl AssocItemKind { pub fn ident (& self) -> Option < Ident > { match * self { AssocItemKind :: Const (box ConstItem { ident , .. }) | AssocItemKind :: Fn (box Fn { ident , .. }) | AssocItemKind :: Type (box TyAlias { ident , .. }) | AssocItemKind :: Delegation (box Delegation { ident , .. }) => Some (ident) , AssocItemKind :: MacCall (_) | AssocItemKind :: DelegationMac (_) => None , } } pub fn defaultness (& self) -> Defaultness { match * self { Self :: Const (box ConstItem { defaultness , .. }) | Self :: Fn (box Fn { defaultness , .. }) | Self :: Type (box TyAlias { defaultness , .. }) => defaultness , Self :: MacCall (..) | Self :: Delegation (..) | Self :: DelegationMac (..) => { Defaultness :: Final } } } }
    };
}

impl_230!()