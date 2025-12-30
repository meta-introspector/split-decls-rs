// Generated macro for impl_31 (impl)
macro_rules! Depcrate_heximpl_31 {
() => {
// Module: crate::hex
// Provides: {"impl_31"}
// Dependencies: {}
impl < N : ArrayLength > fmt :: UpperHex for GenericArray < u8 , N > where N : Add < N > , Sum < N , N > : ArrayLength , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { generic_hex :: < _ , true > (self , f) } }
};
}
