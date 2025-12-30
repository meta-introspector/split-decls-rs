// Generated macro for impl_177 (impl)
macro_rules! Depcrate_sync_listimpl_177 {
() => {
// Module: crate::sync::list
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'g , T : 'g , C : IsElement < T > > Iterator for Iter < 'g , T , C > { type Item = Result < & 'g T , IterError > ; fn next (& mut self) -> Option < Self :: Item > { while let Some (c) = unsafe { self . curr . as_ref () } { let succ = c . next . load (Acquire , self . guard) ; if succ . tag () == 1 { let succ = succ . with_tag (0) ; debug_assert ! (self . curr . tag () == 0) ; let succ = match self . pred . compare_exchange (self . curr , succ , Acquire , Acquire , self . guard) { Ok (_) => { unsafe { C :: finalize (self . curr . deref () , self . guard) ; } succ } Err (e) => { e . current } } ; if succ . tag () != 0 { self . pred = self . head ; self . curr = self . head . load (Acquire , self . guard) ; return Some (Err (IterError :: Stalled)) ; } self . curr = succ ; continue ; } self . pred = & c . next ; self . curr = succ ; return Some (Ok (unsafe { C :: element_of (c) })) ; } None } }
};
}
