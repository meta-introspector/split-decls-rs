// Generated macro for impl_7 (impl)
macro_rules! Depcrate_extimpl_7 {
() => {
// Module: crate::ext
// Provides: {"impl_7"}
// Dependencies: {}
impl < 's > Iterator for Split < 's , '_ > { type Item = & 's OsStr ; fn next (& mut self) -> Option < Self :: Item > { let haystack = self . haystack ? ; if let Some ((first , second)) = haystack . split_once (self . needle) { if ! haystack . is_empty () { debug_assert_ne ! (haystack , second) ; } self . haystack = Some (second) ; Some (first) } else { self . haystack = None ; Some (haystack) } } }
};
}
