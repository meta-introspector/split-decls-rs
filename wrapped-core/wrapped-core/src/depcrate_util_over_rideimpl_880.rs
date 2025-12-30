// Generated macro for impl_880 (impl)
macro_rules! Depcrate_util_over_rideimpl_880 {
() => {
// Module: crate::util::over_ride
// Provides: {"impl_880"}
// Dependencies: {}
impl < T : fmt :: Display > fmt :: Display for Override < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Inherit => write ! (f , "Inherit") , Explicit (ref val) => write ! (f , "Explicit `{}`" , val) , } } }
};
}
