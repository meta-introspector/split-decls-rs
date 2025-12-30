// Generated macro for impl_50 (impl)
macro_rules! Depcrate_array_stringimpl_50 {
() => {
// Module: crate::array_string
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "serde")] # [doc = " Requires crate feature `\"serde\"`"] impl < 'de , const CAP : usize > Deserialize < 'de > for ArrayString < CAP > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { use serde :: de :: { self , Visitor } ; use std :: marker :: PhantomData ; struct ArrayStringVisitor < const CAP : usize > (PhantomData < [u8 ; CAP] >) ; impl < 'de , const CAP : usize > Visitor < 'de > for ArrayStringVisitor < CAP > { type Value = ArrayString < CAP > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a string no more than {} bytes long" , CAP) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : de :: Error , { ArrayString :: from (v) . map_err (| _ | E :: invalid_length (v . len () , & self)) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : de :: Error , { let s = str :: from_utf8 (v) . map_err (| _ | E :: invalid_value (de :: Unexpected :: Bytes (v) , & self)) ? ; ArrayString :: from (s) . map_err (| _ | E :: invalid_length (s . len () , & self)) } } deserializer . deserialize_str (ArrayStringVisitor (PhantomData)) } }
};
}
