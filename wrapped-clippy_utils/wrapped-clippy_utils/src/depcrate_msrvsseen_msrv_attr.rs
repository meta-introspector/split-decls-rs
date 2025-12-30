// Generated macro for SEEN_MSRV_ATTR (static)
macro_rules! Depcrate_msrvsSEEN_MSRV_ATTR {
() => {
// Module: crate::msrvs
// Provides: {"SEEN_MSRV_ATTR"}
// Dependencies: {}
# [doc = " `#[clippy::msrv]` attributes are rarely used outside of Clippy's test suite, as a basic"] # [doc = " optimization we can skip traversing the HIR in [`Msrv::meets`] if we never saw an MSRV attribute"] # [doc = " during the early lint passes"] static SEEN_MSRV_ATTR : AtomicBool = AtomicBool :: new (false) ;
};
}
