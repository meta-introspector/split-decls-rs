// Generated macro for impl_254 (impl)
macro_rules! Depcrate_astimpl_254 {
() => {
// Module: crate::ast
// Provides: {"impl_254"}
// Dependencies: {}
impl TryFrom < ItemKind > for AssocItemKind { type Error = ItemKind ; fn try_from (item_kind : ItemKind) -> Result < AssocItemKind , ItemKind > { Ok (match item_kind { ItemKind :: Const (item) => AssocItemKind :: Const (item) , ItemKind :: Fn (fn_kind) => AssocItemKind :: Fn (fn_kind) , ItemKind :: TyAlias (ty_kind) => AssocItemKind :: Type (ty_kind) , ItemKind :: MacCall (a) => AssocItemKind :: MacCall (a) , ItemKind :: Delegation (d) => AssocItemKind :: Delegation (d) , ItemKind :: DelegationMac (d) => AssocItemKind :: DelegationMac (d) , _ => return Err (item_kind) , }) } }
};
}
