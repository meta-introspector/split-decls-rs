// Generated macro for impl_71 (impl)
macro_rules! Depcrate_stringimpl_71 {
() => {
// Module: crate::string
// Provides: {"impl_71"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < B : crate :: backend :: HeapStr > serde :: de :: Visitor < '_ > for StringVisitor < B > { type Value = KStringBase < B > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (Self :: Value :: from_ref (v)) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (Self :: Value :: from_string (v)) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : serde :: de :: Error , { match std :: str :: from_utf8 (v) { Ok (s) => Ok (Self :: Value :: from_ref (s)) , Err (_) => Err (serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Bytes (v) , & self ,)) , } } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : serde :: de :: Error , { match String :: from_utf8 (v) { Ok (s) => Ok (Self :: Value :: from_string (s)) , Err (e) => Err (serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Bytes (& e . into_bytes ()) , & self ,)) , } } }
};
}
