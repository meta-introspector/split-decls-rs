// Generated macro for impl_685 (impl)
macro_rules! Depcrate_tuple_implimpl_685 {
() => {
// Module: crate::tuple_impl
// Provides: {"impl_685"}
// Dependencies: {}
impl < T > Iterator for TupleBuffer < T > where T : HomogeneousTuple , { type Item = T :: Item ; fn next (& mut self) -> Option < Self :: Item > { let s = self . buf . as_mut () ; if let Some (ref mut item) = s . get_mut (self . cur) { self . cur += 1 ; item . take () } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let buffer = & self . buf . as_ref () [self . cur ..] ; let len = if buffer . is_empty () { 0 } else { buffer . iter () . position (| x | x . is_none ()) . unwrap_or (buffer . len ()) } ; (len , Some (len)) } }
};
}
