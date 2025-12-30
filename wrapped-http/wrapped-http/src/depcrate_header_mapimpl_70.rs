// Generated macro for impl_70 (impl)
macro_rules! Depcrate_header_mapimpl_70 {
() => {
// Module: crate::header::map
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = (& 'a HeaderName , & 'a T) ; fn next (& mut self) -> Option < Self :: Item > { use self :: Cursor :: * ; if self . cursor . is_none () { if (self . entry + 1) >= self . map . entries . len () { return None ; } self . entry += 1 ; self . cursor = Some (Cursor :: Head) ; } let entry = & self . map . entries [self . entry] ; match self . cursor . unwrap () { Head => { self . cursor = entry . links . map (| l | Values (l . next)) ; Some ((& entry . key , & entry . value)) } Values (idx) => { let extra = & self . map . extra_values [idx] ; match extra . next { Link :: Entry (_) => self . cursor = None , Link :: Extra (i) => self . cursor = Some (Values (i)) , } Some ((& entry . key , & extra . value)) } } } fn size_hint (& self) -> (usize , Option < usize >) { let map = self . map ; debug_assert ! (map . entries . len () >= self . entry) ; let lower = map . entries . len () - self . entry ; (lower , None) } }
};
}
