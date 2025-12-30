// Generated macro for Directive (enum)
macro_rules! Depcrate_common_cache_controlDirective {
() => {
// Module: crate::common::cache_control
// Provides: {"Directive"}
// Dependencies: {}
# [derive (Clone , Copy)] enum Directive { NoCache , NoStore , NoTransform , OnlyIfCached , MaxAge (u64) , MaxStale (u64) , MinFresh (u64) , MustRevalidate , MustUnderstand , Public , Private , Immutable , ProxyRevalidate , SMaxAge (u64) , }
};
}
