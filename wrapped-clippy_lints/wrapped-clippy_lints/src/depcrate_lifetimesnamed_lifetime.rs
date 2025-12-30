// Generated macro for named_lifetime (function)
macro_rules! Depcrate_lifetimesnamed_lifetime {
() => {
// Module: crate::lifetimes
// Provides: {"named_lifetime"}
// Dependencies: {}
fn named_lifetime (lt : & Lifetime) -> Option < LocalDefId > { match lt . kind { LifetimeKind :: Param (id) if ! lt . is_anonymous () => Some (id) , _ => None , } }
};
}
