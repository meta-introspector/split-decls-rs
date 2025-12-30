// Generated macro for impl_30 (impl)
macro_rules! Depcrate_heximpl_30 {
() => {
// Module: crate::hex
// Provides: {"impl_30"}
// Dependencies: {}
impl < N : ArrayLength > fmt :: LowerHex for GenericArray < u8 , N > where N : Add < N > , Sum < N , N > : ArrayLength , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { generic_hex :: < _ , false > (self , f) } }
};
}
