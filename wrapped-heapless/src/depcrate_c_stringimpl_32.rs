// Generated macro for impl_32 (impl)
macro_rules! Depcrate_c_stringimpl_32 {
() => {
// Module: crate::c_string
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: Display for ExtendError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Capacity (error) => write ! (f , "{error}") , Self :: InteriorNul { position } => write ! (f , "interior nul byte at {position}") , } } }
};
}
