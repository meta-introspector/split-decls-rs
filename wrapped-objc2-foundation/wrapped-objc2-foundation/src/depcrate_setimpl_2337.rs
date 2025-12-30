// Generated macro for impl_2337 (impl)
macro_rules! Depcrate_setimpl_2337 {
() => {
// Module: crate::set
// Provides: {"impl_2337"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] impl < ObjectType : fmt :: Debug + Message > fmt :: Debug for NSSet < ObjectType > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self) . finish () } }
};
}
