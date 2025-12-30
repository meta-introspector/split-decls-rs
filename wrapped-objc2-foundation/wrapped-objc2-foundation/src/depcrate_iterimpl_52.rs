// Generated macro for impl_52 (impl)
macro_rules! Depcrate_iterimpl_52 {
() => {
// Module: crate::iter
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a , C : FastEnumerationHelper > Iterator for IterUnchecked < 'a , C > { type Item = & 'a C :: Item ; # [inline] # [track_caller] fn next (& mut self) -> Option < & 'a C :: Item > { # [cfg (debug_assertions)] let mutations_state = Some (& mut self . mutations_state) ; # [cfg (not (debug_assertions))] let mutations_state = None ; let obj = unsafe { self . helper . next_from (ProtocolObject :: from_ref (self . collection) , mutations_state) ? } ; Some (unsafe { obj . cast :: < C :: Item > () . as_ref () }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . helper . remaining_items_at_least () , self . collection . maybe_len () ,) } }
};
}
