// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < B : BitBlock > fmt :: Debug for BitSet < B > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("BitSet") . field ("bit_vec" , & self . bit_vec) . finish () } }
};
}
