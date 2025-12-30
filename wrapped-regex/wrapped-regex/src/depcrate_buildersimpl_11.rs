// Generated macro for impl_11 (impl)
macro_rules! Depcrate_buildersimpl_11 {
() => {
// Module: crate::builders
// Provides: {"impl_11"}
// Dependencies: {}
impl Default for Builder { fn default () -> Builder { let metac = meta :: Config :: new () . nfa_size_limit (Some (10 * (1 << 20))) . hybrid_cache_capacity (2 * (1 << 20)) ; Builder { pats : vec ! [] , metac , syntaxc : syntax :: Config :: default () } } }
};
}
