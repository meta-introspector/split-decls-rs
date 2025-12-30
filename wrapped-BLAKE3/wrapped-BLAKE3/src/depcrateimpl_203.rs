// Generated macro for impl_203 (impl)
macro_rules! Depcrateimpl_203 {
() => {
// Module: crate
// Provides: {"impl_203"}
// Dependencies: {}
impl fmt :: Display for Hash { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let hex = self . to_hex () ; let hex : & str = hex . as_str () ; f . write_str (hex) } }
};
}
