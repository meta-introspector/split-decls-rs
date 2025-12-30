// Generated macro for impl_890 (impl)
macro_rules! Depcrate_util_primitivesimpl_890 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_890"}
// Dependencies: {}
impl core :: fmt :: Display for SmallIndexError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to create small index from {:?}, which exceeds {:?}" , self . attempted () , SmallIndex :: MAX ,) } }
};
}
