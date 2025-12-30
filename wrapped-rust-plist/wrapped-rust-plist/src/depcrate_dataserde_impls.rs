// Generated macro for serde_impls (module)
macro_rules! Depcrate_dataserde_impls {
() => {
// Module: crate::data
// Provides: {"serde_impls"}
// Dependencies: {}
pub mod serde_impls { use serde :: { de , ser } ; use std :: fmt ; use crate :: Data ; impl ser :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serializer . serialize_bytes (self . as_ref ()) } } struct DataVisitor ; impl de :: Visitor < '_ > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a byte array") } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : de :: Error , { self . visit_byte_buf (v . to_owned ()) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : de :: Error , { Ok (v . into ()) } } impl < 'de > de :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_byte_buf (DataVisitor) } } }
};
}
