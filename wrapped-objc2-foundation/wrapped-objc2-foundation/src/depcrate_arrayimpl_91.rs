// Generated macro for impl_91 (impl)
macro_rules! Depcrate_arrayimpl_91 {
() => {
// Module: crate::array
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] impl < ObjectType : fmt :: Debug + Message > fmt :: Debug for NSArray < ObjectType > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self) . finish () } }
};
}
