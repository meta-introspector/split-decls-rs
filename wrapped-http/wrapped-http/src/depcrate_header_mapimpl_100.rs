// Generated macro for impl_100 (impl)
macro_rules! Depcrate_header_mapimpl_100 {
() => {
// Module: crate::header::map
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a , T : 'a > Iterator for ValueIterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { use self :: Cursor :: * ; let entry = & mut unsafe { & mut * self . map } . entries [self . index] ; match self . front { Some (Head) => { if self . back == Some (Head) { self . front = None ; self . back = None ; } else { match entry . links { Some (links) => { self . front = Some (Values (links . next)) ; } None => unreachable ! () , } } Some (& mut entry . value) } Some (Values (idx)) => { let extra = & mut unsafe { & mut * self . map } . extra_values [idx] ; if self . front == self . back { self . front = None ; self . back = None ; } else { match extra . next { Link :: Entry (_) => self . front = None , Link :: Extra (i) => self . front = Some (Values (i)) , } } Some (& mut extra . value) } None => None , } } }
};
}
