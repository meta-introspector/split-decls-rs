// Generated macro for impl_34 (impl)
macro_rules! Depcrate_ast_utils_ident_iterimpl_34 {
() => {
// Module: crate::ast_utils::ident_iter
// Provides: {"impl_34"}
// Dependencies: {}
impl From < & Attribute > for IdentIter { fn from (attr : & Attribute) -> Self { let mut visitor = IdentCollector :: default () ; walk_attribute (& mut visitor , attr) ; IdentIter (visitor . 0 . into_iter ()) } }
};
}
