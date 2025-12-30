// Generated macro for impl_65 (impl)
macro_rules! Depcrate_endianimpl_65 {
() => {
// Module: crate::endian
// Provides: {"impl_65"}
// Dependencies: {}
impl < E : Endian > fmt :: Debug for U32Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "U32({:x}, {:x}, {:x}, {:x})" , self . 0 [0] , self . 0 [1] , self . 0 [2] , self . 0 [3] ,) } }
};
}
