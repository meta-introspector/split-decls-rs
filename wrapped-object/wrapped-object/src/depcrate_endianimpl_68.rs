// Generated macro for impl_68 (impl)
macro_rules! Depcrate_endianimpl_68 {
() => {
// Module: crate::endian
// Provides: {"impl_68"}
// Dependencies: {}
impl < E : Endian > fmt :: Debug for I32Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "I32({:x}, {:x}, {:x}, {:x})" , self . 0 [0] , self . 0 [1] , self . 0 [2] , self . 0 [3] ,) } }
};
}
