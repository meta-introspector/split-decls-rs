// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < B : BitBlock > fmt :: Display for BitSet < B > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_set () . entries (self) . finish () } }
};
}
