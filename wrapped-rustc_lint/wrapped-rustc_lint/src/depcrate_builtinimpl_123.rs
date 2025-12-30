// Generated macro for impl_123 (impl)
macro_rules! Depcrate_builtinimpl_123 {
() => {
// Module: crate::builtin
// Provides: {"impl_123"}
// Dependencies: {}
impl hir :: intravisit :: Visitor < '_ > for ShorthandAssocTyCollector { fn visit_qpath (& mut self , qpath : & hir :: QPath < '_ > , id : hir :: HirId , _ : Span) { if let hir :: QPath :: TypeRelative (qself , _) = qpath && qself . as_generic_param () . is_some () { self . qselves . push (qself . span) ; } hir :: intravisit :: walk_qpath (self , qpath , id) } }
};
}
