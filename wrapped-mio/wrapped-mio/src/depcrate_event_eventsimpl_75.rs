// Generated macro for impl_75 (impl)
macro_rules! Depcrate_event_eventsimpl_75 {
() => {
// Module: crate::event::events
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = & 'a Event ; fn next (& mut self) -> Option < Self :: Item > { let ret = self . inner . inner . get (self . pos) . map (Event :: from_sys_event_ref) ; self . pos += 1 ; ret } fn size_hint (& self) -> (usize , Option < usize >) { let size = self . inner . inner . len () ; (size , Some (size)) } fn count (self) -> usize { self . inner . inner . len () } }
};
}
