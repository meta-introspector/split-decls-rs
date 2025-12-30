// Generated macro for impl_722 (impl)
macro_rules! Depcrate_fipsimpl_722 {
() => {
// Module: crate::fips
// Provides: {"impl_722"}
// Dependencies: {}
# [cfg (all (feature = "fips" , debug_assertions))] impl < R > FipsServiceStatus < R > { # [doc = " Maps a `ServiceStatus<R>` to a `ServiceStatus<S>` by applying a function to a contained value."] # [allow (dead_code)] pub fn map < S , F > (self , op : F) -> FipsServiceStatus < S > where F : FnOnce (R) -> S , { match self { FipsServiceStatus :: Approved (v) => FipsServiceStatus :: Approved (op (v)) , FipsServiceStatus :: NonApproved (v) => FipsServiceStatus :: NonApproved (op (v)) , FipsServiceStatus :: Unset (v) => FipsServiceStatus :: Unset (op (v)) , } } }
};
}
