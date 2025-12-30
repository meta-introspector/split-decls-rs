// Generated macro for MacroInvocationNode (trait)
macro_rules! Depcrate_ast_traitsMacroInvocationNode {
() => {
// Module: crate::ast_traits
// Provides: {"MacroInvocationNode"}
// Dependencies: {}
pub trait MacroInvocationNode : HasAttrs + HasNodeId + Sized { type ItemKind ; fn is_mac_call (& self) -> bool ; fn take_mac_call (self) -> (ast :: MacCall , AttrVec , AddSemicolon) ; fn delegation (& self) -> Option < (& ast :: MacCall , & ast :: AssocItem) > ; fn delegation_item_kind (_deleg : Box < ast :: Delegation >) -> Self :: ItemKind ; fn from_item (_item : ast :: Item < Self :: ItemKind >) -> Self ; }
};
}
