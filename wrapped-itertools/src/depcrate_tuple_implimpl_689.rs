// Generated macro for impl_689 (impl)
macro_rules! Depcrate_tuple_implimpl_689 {
() => {
// Module: crate::tuple_impl
// Provides: {"impl_689"}
// Dependencies: {}
impl < I , T > Iterator for Tuples < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { T :: collect_from_iter (& mut self . iter , & mut self . buf) } fn size_hint (& self) -> (usize , Option < usize >) { let buffered = T :: buffer_len (& self . buf) ; let (unbuffered_lo , unbuffered_hi) = self . iter . size_hint () ; let total_lo = add_then_div (unbuffered_lo , buffered , T :: num_items ()) . unwrap_or (usize :: MAX) ; let total_hi = unbuffered_hi . and_then (| hi | add_then_div (hi , buffered , T :: num_items ())) ; (total_lo , total_hi) } }
};
}
