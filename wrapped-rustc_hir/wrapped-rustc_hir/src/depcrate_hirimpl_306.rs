// Generated macro for impl_306 (impl)
macro_rules! Depcrate_hirimpl_306 {
() => {
// Module: crate::hir
// Provides: {"impl_306"}
// Dependencies: {}
impl BodyOwnerKind { pub fn is_fn_or_closure (self) -> bool { match self { BodyOwnerKind :: Fn | BodyOwnerKind :: Closure => true , BodyOwnerKind :: Const { .. } | BodyOwnerKind :: Static (_) | BodyOwnerKind :: GlobalAsm => { false } } } }
};
}
