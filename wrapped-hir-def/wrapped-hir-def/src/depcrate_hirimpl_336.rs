// Generated macro for impl_336 (impl)
macro_rules! Depcrate_hirimpl_336 {
() => {
// Module: crate::hir
// Provides: {"impl_336"}
// Dependencies: {}
impl Pat { pub fn walk_child_pats (& self , mut f : impl FnMut (PatId)) { match self { Pat :: Range { .. } | Pat :: Lit (..) | Pat :: Path (..) | Pat :: ConstBlock (..) | Pat :: Wild | Pat :: Missing | Pat :: Expr (_) => { } Pat :: Bind { subpat , .. } => { subpat . iter () . copied () . for_each (f) ; } Pat :: Or (args) | Pat :: Tuple { args , .. } | Pat :: TupleStruct { args , .. } => { args . iter () . copied () . for_each (f) ; } Pat :: Ref { pat , .. } => f (* pat) , Pat :: Slice { prefix , slice , suffix } => { let total_iter = prefix . iter () . chain (slice . iter ()) . chain (suffix . iter ()) ; total_iter . copied () . for_each (f) ; } Pat :: Record { args , .. } => { args . iter () . map (| f | f . pat) . for_each (f) ; } Pat :: Box { inner } => f (* inner) , } } }
};
}
