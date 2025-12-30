// Generated macro for impl_469 (impl)
macro_rules! Depcrate_util_primitivesimpl_469 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_469"}
// Dependencies: {}
impl core :: fmt :: Display for SmallIndexError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to create small index from {:?}, which exceeds {:?}" , self . attempted () , SmallIndex :: MAX ,) } }
};
}
