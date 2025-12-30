// Generated macro for impl_33 (impl)
macro_rules! Depcrate_cqueueimpl_33 {
() => {
// Module: crate::cqueue
// Provides: {"impl_33"}
// Dependencies: {}
impl < E : EntryMarker > Iterator for CompletionQueue < '_ , E > { type Item = E ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . head != self . tail { Some (unsafe { self . pop () }) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
