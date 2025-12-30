// Generated macro for impl_64 (impl)
macro_rules! Depcrate_endianimpl_64 {
() => {
// Module: crate::endian
// Provides: {"impl_64"}
// Dependencies: {}
impl < E : Endian > fmt :: Debug for U16Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "U16({:x}, {:x})" , self . 0 [0] , self . 0 [1] ,) } }
};
}
