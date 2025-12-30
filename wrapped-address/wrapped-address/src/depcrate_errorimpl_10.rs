// Generated macro for impl_10 (impl)
macro_rules! Depcrate_errorimpl_10 {
() => {
// Module: crate::error
// Provides: {"impl_10"}
// Dependencies: {}
# [cfg (feature = "decode")] impl fmt :: Display for ParseAddressError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { ParseAddressError :: WrongSize => f . write_str ("String is the wrong size") , ParseAddressError :: Invalid => f . write_str ("Invalid Base58 string") , } } }
};
}
