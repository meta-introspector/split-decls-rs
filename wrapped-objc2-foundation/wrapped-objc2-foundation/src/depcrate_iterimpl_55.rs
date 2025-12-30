// Generated macro for impl_55 (impl)
macro_rules! Depcrate_iterimpl_55 {
() => {
// Module: crate::iter
// Provides: {"impl_55"}
// Dependencies: {}
impl < C : FastEnumerationHelper > Iterator for Iter < '_ , C > { type Item = Retained < C :: Item > ; # [inline] # [track_caller] fn next (& mut self) -> Option < Retained < C :: Item > > { let obj = unsafe { self . helper . next_from (ProtocolObject :: from_ref (self . collection) , Some (& mut self . mutations_state) ,) ? } ; Some (unsafe { obj . cast :: < C :: Item > () . as_ref () } . retain ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . helper . remaining_items_at_least () , self . collection . maybe_len () ,) } }
};
}
