// Generated macro for impl_809 (impl)
macro_rules! Depcrate_prefixesimpl_809 {
() => {
// Module: crate::prefixes
// Provides: {"impl_809"}
// Dependencies: {}
impl < 'tcx > Iterator for Prefixes < 'tcx > { type Item = PlaceRef < 'tcx > ; fn next (& mut self) -> Option < Self :: Item > { let mut cursor = self . next ? ; loop { match cursor . last_projection () { None => { self . next = None ; return Some (cursor) ; } Some ((cursor_base , elem)) => { match elem { ProjectionElem :: Field (_ , _) => { self . next = Some (cursor_base) ; return Some (cursor) ; } ProjectionElem :: UnwrapUnsafeBinder (_) => { self . next = Some (cursor_base) ; return Some (cursor) ; } ProjectionElem :: Downcast (..) | ProjectionElem :: Subslice { .. } | ProjectionElem :: OpaqueCast { .. } | ProjectionElem :: ConstantIndex { .. } | ProjectionElem :: Index (_) => { cursor = cursor_base ; } ProjectionElem :: Subtype (..) => { panic ! ("Subtype projection is not allowed before borrow check") } ProjectionElem :: Deref => { match self . kind { PrefixSet :: Shallow => { self . next = None ; return Some (cursor) ; } PrefixSet :: All => { self . next = Some (cursor_base) ; return Some (cursor) ; } } } } } } } } }
};
}
