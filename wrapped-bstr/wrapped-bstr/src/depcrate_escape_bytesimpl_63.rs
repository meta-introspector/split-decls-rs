// Generated macro for impl_63 (impl)
macro_rules! Depcrate_escape_bytesimpl_63 {
() => {
// Module: crate::escape_bytes
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > core :: fmt :: Display for EscapeBytes < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use core :: fmt :: Write ; for ch in self . clone () { f . write_char (ch) ? ; } Ok (()) } }
};
}
