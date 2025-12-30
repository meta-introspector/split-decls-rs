// Generated macro for impl_58 (impl)
macro_rules! Depcrate_iterimpl_58 {
() => {
// Module: crate::iter
// Provides: {"impl_58"}
// Dependencies: {}
impl < C : FastEnumerationHelper > Iterator for IntoIter < C > { type Item = Retained < C :: Item > ; # [inline] # [track_caller] fn next (& mut self) -> Option < Retained < C :: Item > > { let collection = ProtocolObject :: from_ref (& * self . collection) ; let obj = unsafe { self . helper . next_from (collection , Some (& mut self . mutations_state)) ? } ; let obj = unsafe { Retained :: retain (obj . cast :: < C :: Item > () . as_ptr ()) } ; Some (unsafe { obj . unwrap_unchecked () }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . helper . remaining_items_at_least () , self . collection . maybe_len () ,) } }
};
}
