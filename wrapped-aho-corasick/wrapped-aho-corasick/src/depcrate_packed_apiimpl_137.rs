// Generated macro for impl_137 (impl)
macro_rules! Depcrate_packed_apiimpl_137 {
() => {
// Module: crate::packed::api
// Provides: {"impl_137"}
// Dependencies: {}
impl < 's , 'h > Iterator for FindIter < 's , 'h > { type Item = Match ; fn next (& mut self) -> Option < Match > { if self . span . start > self . span . end { return None ; } match self . searcher . find_in (self . haystack , self . span) { None => None , Some (m) => { self . span . start = m . end () ; Some (m) } } } }
};
}
