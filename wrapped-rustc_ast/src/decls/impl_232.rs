macro_rules! deps {
    () => {
        TyAlias!();
        Delegation!();
        DelegationMac!();
        Const!();
        AssocItemKind!();
        Fn!();
        Type!();
        ItemKind!();
        MacCall!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl TryFrom < ItemKind > for AssocItemKind { type Error = ItemKind ; fn try_from (item_kind : ItemKind) -> Result < AssocItemKind , ItemKind > { Ok (match item_kind { ItemKind :: Const (item) => AssocItemKind :: Const (item) , ItemKind :: Fn (fn_kind) => AssocItemKind :: Fn (fn_kind) , ItemKind :: TyAlias (ty_kind) => AssocItemKind :: Type (ty_kind) , ItemKind :: MacCall (a) => AssocItemKind :: MacCall (a) , ItemKind :: Delegation (d) => AssocItemKind :: Delegation (d) , ItemKind :: DelegationMac (d) => AssocItemKind :: DelegationMac (d) , _ => return Err (item_kind) , }) } }
    };
}

impl_232!();