// Generated macro for impl_101 (impl)
macro_rules! Depcrate_header_mapimpl_101 {
() => {
// Module: crate::header::map
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a , T : 'a > DoubleEndedIterator for ValueIterMut < 'a , T > { fn next_back (& mut self) -> Option < Self :: Item > { use self :: Cursor :: * ; let entry = & mut unsafe { & mut * self . map } . entries [self . index] ; match self . back { Some (Head) => { self . front = None ; self . back = None ; Some (& mut entry . value) } Some (Values (idx)) => { let extra = & mut unsafe { & mut * self . map } . extra_values [idx] ; if self . front == self . back { self . front = None ; self . back = None ; } else { match extra . prev { Link :: Entry (_) => self . back = Some (Head) , Link :: Extra (idx) => self . back = Some (Values (idx)) , } } Some (& mut extra . value) } None => None , } } }
};
}
