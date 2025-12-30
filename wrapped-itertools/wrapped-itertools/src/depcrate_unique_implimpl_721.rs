// Generated macro for impl_721 (impl)
macro_rules! Depcrate_unique_implimpl_721 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_721"}
// Dependencies: {}
impl < I > DoubleEndedIterator for Unique < I > where I : DoubleEndedIterator , I :: Item : Eq + Hash + Clone , { fn next_back (& mut self) -> Option < Self :: Item > { let UniqueBy { iter , used , .. } = & mut self . iter ; iter . rev () . find_map (| v | { if let Entry :: Vacant (entry) = used . entry (v) { let elt = entry . key () . clone () ; entry . insert (()) ; return Some (elt) ; } None }) } }
};
}
