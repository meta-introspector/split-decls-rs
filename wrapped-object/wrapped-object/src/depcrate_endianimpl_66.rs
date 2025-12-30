// Generated macro for impl_66 (impl)
macro_rules! Depcrate_endianimpl_66 {
() => {
// Module: crate::endian
// Provides: {"impl_66"}
// Dependencies: {}
impl < E : Endian > fmt :: Debug for U64Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "U64({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})" , self . 0 [0] , self . 0 [1] , self . 0 [2] , self . 0 [3] , self . 0 [4] , self . 0 [5] , self . 0 [6] , self . 0 [7] ,) } }
};
}
