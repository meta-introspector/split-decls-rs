// Generated macro for impl_15 (impl)
macro_rules! Depcrate_errorsimpl_15 {
() => {
// Module: crate::errors
// Provides: {"impl_15"}
// Dependencies: {}
impl fmt :: Display for InvalidValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> core :: result :: Result < () , fmt :: Error > { match self { Self :: InvalidChar (c) => write ! (f , "contains invalid character: '{c}'") , Self :: InvalidFormat => f . write_str ("value format is invalid") , Self :: Malformed => f . write_str ("value malformed") , Self :: TooLong => f . write_str ("value to long") , Self :: TooShort => f . write_str ("value to short") , } } }
};
}
