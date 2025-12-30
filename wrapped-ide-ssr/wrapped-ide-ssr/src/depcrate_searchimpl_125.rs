// Generated macro for impl_125 (impl)
macro_rules! Depcrate_searchimpl_125 {
() => {
// Module: crate::search
// Provides: {"impl_125"}
// Dependencies: {}
impl UsageCache { fn find (& mut self , definition : & Definition) -> Option < & UsageSearchResult > { for (d , refs) in & self . usages { if d == definition { return Some (refs) ; } } None } }
};
}
