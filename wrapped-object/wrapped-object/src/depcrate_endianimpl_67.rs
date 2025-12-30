// Generated macro for impl_67 (impl)
macro_rules! Depcrate_endianimpl_67 {
() => {
// Module: crate::endian
// Provides: {"impl_67"}
// Dependencies: {}
impl < E : Endian > fmt :: Debug for I16Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "I16({:x}, {:x})" , self . 0 [0] , self . 0 [1] ,) } }
};
}
