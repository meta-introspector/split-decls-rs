// Generated macro for impl_97 (impl)
macro_rules! Depcrate_header_mapimpl_97 {
() => {
// Module: crate::header::map
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'a , T : 'a > Iterator for ValueIter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { use self :: Cursor :: * ; match self . front { Some (Head) => { let entry = & self . map . entries [self . index] ; if self . back == Some (Head) { self . front = None ; self . back = None ; } else { match entry . links { Some (links) => { self . front = Some (Values (links . next)) ; } None => unreachable ! () , } } Some (& entry . value) } Some (Values (idx)) => { let extra = & self . map . extra_values [idx] ; if self . front == self . back { self . front = None ; self . back = None ; } else { match extra . next { Link :: Entry (_) => self . front = None , Link :: Extra (i) => self . front = Some (Values (i)) , } } Some (& extra . value) } None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { match (self . front , self . back) { (Some (Cursor :: Head) , Some (Cursor :: Head)) => (1 , Some (1)) , (Some (_) , _) => (1 , None) , (None , _) => (0 , Some (0)) , } } }
};
}
