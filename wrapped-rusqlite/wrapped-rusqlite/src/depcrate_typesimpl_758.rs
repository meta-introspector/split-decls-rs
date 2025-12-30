// Generated macro for impl_758 (impl)
macro_rules! Depcrate_typesimpl_758 {
() => {
// Module: crate::types
// Provides: {"impl_758"}
// Dependencies: {}
impl fmt :: Display for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Null => f . pad ("Null") , Self :: Integer => f . pad ("Integer") , Self :: Real => f . pad ("Real") , Self :: Text => f . pad ("Text") , Self :: Blob => f . pad ("Blob") , } } }
};
}
