// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl < B : BitBlock > fmt :: Display for BitVec < B > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { self . ensure_invariant () ; for bit in self { fmt . write_char (if bit { '1' } else { '0' }) ? ; } Ok (()) } }
};
}
