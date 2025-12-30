// Generated macro for impl_2714 (impl)
macro_rules! Depcrate_fnsimpl_2714 {
() => {
// Module: crate::fns
// Provides: {"impl_2714"}
// Dependencies: {}
impl < T > FnOnce1 < Result < T , T > > for MergeResultFn { type Output = T ; fn call_once (self , arg : Result < T , T >) -> Self :: Output { match arg { Ok (x) => x , Err (x) => x , } } }
};
}
