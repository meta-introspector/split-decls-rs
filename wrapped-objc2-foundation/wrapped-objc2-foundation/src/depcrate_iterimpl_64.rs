// Generated macro for impl_64 (impl)
macro_rules! Depcrate_iterimpl_64 {
() => {
// Module: crate::iter
// Provides: {"impl_64"}
// Dependencies: {}
impl < C , E > Iterator for IterWithBackingEnum < '_ , C , E > where C : ? Sized + FastEnumerationHelper , E : FastEnumerationHelper , { type Item = Retained < E :: Item > ; # [inline] # [track_caller] fn next (& mut self) -> Option < Retained < E :: Item > > { let obj = unsafe { self . helper . next_from (ProtocolObject :: from_ref (& * self . enumerator) , Some (& mut self . mutations_state) ,) ? } ; Some (unsafe { obj . cast :: < E :: Item > () . as_ref () } . retain ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . helper . remaining_items_at_least () , self . collection . maybe_len () ,) } }
};
}
