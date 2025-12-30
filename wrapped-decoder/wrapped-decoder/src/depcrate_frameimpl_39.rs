// Generated macro for impl_39 (impl)
macro_rules! Depcrate_frameimpl_39 {
() => {
// Module: crate::frame
// Provides: {"impl_39"}
// Dependencies: {}
impl std :: fmt :: UpperHex for I128Hex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 1 { Type :: I8 => fmt :: UpperHex :: fmt (& (self . 0 as i8) , f) , Type :: I16 => fmt :: UpperHex :: fmt (& (self . 0 as i16) , f) , Type :: I32 => fmt :: UpperHex :: fmt (& (self . 0 as i32) , f) , Type :: I64 => fmt :: UpperHex :: fmt (& (self . 0 as i64) , f) , Type :: I128 => fmt :: UpperHex :: fmt (& self . 0 , f) , _ => panic ! ("Unsupported type '{:?}' found." , self . 1) , } } }
};
}
