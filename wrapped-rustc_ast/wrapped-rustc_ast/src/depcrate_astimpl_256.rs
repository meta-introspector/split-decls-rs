// Generated macro for impl_256 (impl)
macro_rules! Depcrate_astimpl_256 {
() => {
// Module: crate::ast
// Provides: {"impl_256"}
// Dependencies: {}
impl ForeignItemKind { pub fn ident (& self) -> Option < Ident > { match * self { ForeignItemKind :: Static (box StaticItem { ident , .. }) | ForeignItemKind :: Fn (box Fn { ident , .. }) | ForeignItemKind :: TyAlias (box TyAlias { ident , .. }) => Some (ident) , ForeignItemKind :: MacCall (_) => None , } } }
};
}
