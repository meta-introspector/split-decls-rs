// Generated macro for SFnPtrMap (type)
macro_rules! Depcrate_arbitrarySFnPtrMap {
() => {
// Module: crate::arbitrary
// Provides: {"SFnPtrMap"}
// Dependencies: {}
pub (crate) type SFnPtrMap < S , O > = statics :: Map < S , fn (< S as Strategy > :: Value) -> O > ;
};
}
