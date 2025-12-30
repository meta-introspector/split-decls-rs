// Generated macro for impl_38 (impl)
macro_rules! Depcrate_frameimpl_38 {
() => {
// Module: crate::frame
// Provides: {"impl_38"}
// Dependencies: {}
impl std :: fmt :: LowerHex for I128Hex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 1 { Type :: I8 => fmt :: LowerHex :: fmt (& (self . 0 as i8) , f) , Type :: I16 => fmt :: LowerHex :: fmt (& (self . 0 as i16) , f) , Type :: I32 => fmt :: LowerHex :: fmt (& (self . 0 as i32) , f) , Type :: I64 => fmt :: LowerHex :: fmt (& (self . 0 as i64) , f) , Type :: I128 => fmt :: LowerHex :: fmt (& self . 0 , f) , _ => panic ! ("Unsupported type '{:?}' found." , self . 1) , } } }
};
}
