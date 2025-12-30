// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (feature = "decode")] impl fmt :: Display for ParseHashError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { ParseHashError :: WrongSize => f . write_str ("string decoded to wrong size for hash") , ParseHashError :: Invalid => f . write_str ("failed to decoded string to hash") , } } }
};
}
