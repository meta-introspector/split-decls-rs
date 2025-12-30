// Generated macro for serde_impls (module)
macro_rules! Depcrate_integerserde_impls {
() => {
// Module: crate::integer
// Provides: {"serde_impls"}
// Dependencies: {}
# [cfg (feature = "serde")] pub mod serde_impls { use serde :: { de :: { Deserialize , Deserializer , Error , Visitor } , ser :: { Serialize , Serializer } , } ; use std :: fmt ; use crate :: Integer ; impl Serialize for Integer { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if let Some (v) = self . as_unsigned () { serializer . serialize_u64 (v) } else if let Some (v) = self . as_signed () { serializer . serialize_i64 (v) } else { unreachable ! () ; } } } struct IntegerVisitor ; impl Visitor < '_ > for IntegerVisitor { type Value = Integer ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a plist integer") } fn visit_i64 < E > (self , v : i64) -> Result < Self :: Value , E > where E : Error , { Ok (Integer :: from (v)) } fn visit_u64 < E > (self , v : u64) -> Result < Self :: Value , E > where E : Error , { Ok (Integer :: from (v)) } } impl < 'de > Deserialize < 'de > for Integer { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_any (IntegerVisitor) } } }
};
}
