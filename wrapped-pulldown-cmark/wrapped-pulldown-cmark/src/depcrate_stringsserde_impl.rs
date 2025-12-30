// Generated macro for serde_impl (module)
macro_rules! Depcrate_stringsserde_impl {
() => {
// Module: crate::strings
// Provides: {"serde_impl"}
// Dependencies: {}
# [cfg (feature = "serde")] mod serde_impl { use core :: fmt ; use serde :: { de , Deserialize , Deserializer , Serialize , Serializer } ; use super :: CowStr ; impl < 'a > Serialize for CowStr < 'a > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self . as_ref ()) } } struct CowStrVisitor ; impl < 'de > de :: Visitor < 'de > for CowStrVisitor { type Value = CowStr < 'de > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a string") } fn visit_borrowed_str < E > (self , v : & 'de str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (CowStr :: Borrowed (v)) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : de :: Error , { match v . try_into () { Ok (it) => Ok (CowStr :: Inlined (it)) , Err (_) => Ok (CowStr :: Boxed (String :: from (v) . into_boxed_str ())) , } } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : de :: Error , { Ok (CowStr :: Boxed (v . into_boxed_str ())) } } impl < 'a , 'de : 'a > Deserialize < 'de > for CowStr < 'a > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (CowStrVisitor) } } }
};
}
