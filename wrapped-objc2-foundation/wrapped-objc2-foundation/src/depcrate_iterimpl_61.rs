// Generated macro for impl_61 (impl)
macro_rules! Depcrate_iterimpl_61 {
() => {
// Module: crate::iter
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , C , E > Iterator for IterUncheckedWithBackingEnum < 'a , C , E > where C : ? Sized + FastEnumerationHelper , E : FastEnumerationHelper , { type Item = & 'a E :: Item ; # [inline] # [track_caller] fn next (& mut self) -> Option < & 'a E :: Item > { # [cfg (debug_assertions)] let mutations_state = Some (& mut self . mutations_state) ; # [cfg (not (debug_assertions))] let mutations_state = None ; let obj = unsafe { self . helper . next_from (ProtocolObject :: from_ref (& * self . enumerator) , mutations_state) ? } ; Some (unsafe { obj . cast :: < E :: Item > () . as_ref () }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . helper . remaining_items_at_least () , self . collection . maybe_len () ,) } }
};
}
