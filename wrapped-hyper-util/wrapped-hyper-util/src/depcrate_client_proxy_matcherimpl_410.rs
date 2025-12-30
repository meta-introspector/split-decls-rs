// Generated macro for impl_410 (impl)
macro_rules! Depcrate_client_proxy_matcherimpl_410 {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"impl_410"}
// Dependencies: {}
impl DomainMatcher { fn contains (& self , domain : & str) -> bool { let domain_len = domain . len () ; for d in & self . 0 { if d == domain || d . strip_prefix ('.') == Some (domain) { return true ; } else if domain . ends_with (d) { if d . starts_with ('.') { return true ; } else if domain . as_bytes () . get (domain_len - d . len () - 1) == Some (& b'.') { return true ; } } else if d == "*" { return true ; } } false } }
};
}
