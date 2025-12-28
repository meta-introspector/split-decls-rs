macro_rules! deps {
    () => {
        TyAlias!();
        Fn!();
        MacCall!();
        ItemKind!();
        ForeignItemKind!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl TryFrom < ItemKind > for ForeignItemKind { type Error = ItemKind ; fn try_from (item_kind : ItemKind) -> Result < ForeignItemKind , ItemKind > { Ok (match item_kind { ItemKind :: Static (box static_item) => ForeignItemKind :: Static (Box :: new (static_item)) , ItemKind :: Fn (fn_kind) => ForeignItemKind :: Fn (fn_kind) , ItemKind :: TyAlias (ty_alias_kind) => ForeignItemKind :: TyAlias (ty_alias_kind) , ItemKind :: MacCall (a) => ForeignItemKind :: MacCall (a) , _ => return Err (item_kind) , }) } }
    };
}

impl_236!();