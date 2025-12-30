// Generated macro for impl_252 (impl)
macro_rules! Depcrate_astimpl_252 {
() => {
// Module: crate::ast
// Provides: {"impl_252"}
// Dependencies: {}
impl AssocItemKind { pub fn ident (& self) -> Option < Ident > { match * self { AssocItemKind :: Const (box ConstItem { ident , .. }) | AssocItemKind :: Fn (box Fn { ident , .. }) | AssocItemKind :: Type (box TyAlias { ident , .. }) | AssocItemKind :: Delegation (box Delegation { ident , .. }) => Some (ident) , AssocItemKind :: MacCall (_) | AssocItemKind :: DelegationMac (_) => None , } } pub fn defaultness (& self) -> Defaultness { match * self { Self :: Const (box ConstItem { defaultness , .. }) | Self :: Fn (box Fn { defaultness , .. }) | Self :: Type (box TyAlias { defaultness , .. }) => defaultness , Self :: MacCall (..) | Self :: Delegation (..) | Self :: DelegationMac (..) => { Defaultness :: Final } } } }
};
}
