// Generated macro for impl_720 (impl)
macro_rules! Depcrate_unique_implimpl_720 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_720"}
// Dependencies: {}
impl < I > Iterator for Unique < I > where I : Iterator , I :: Item : Eq + Hash + Clone , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let UniqueBy { iter , used , .. } = & mut self . iter ; iter . find_map (| v | { if let Entry :: Vacant (entry) = used . entry (v) { let elt = entry . key () . clone () ; entry . insert (()) ; return Some (elt) ; } None }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (low , hi) = self . iter . iter . size_hint () ; ((low > 0 && self . iter . used . is_empty ()) as usize , hi) } fn count (self) -> usize { count_new_keys (self . iter . used , self . iter . iter) } }
};
}
