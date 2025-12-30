// Generated macro for serde_impls (module)
macro_rules! Depcrate_uidserde_impls {
() => {
// Module: crate::uid
// Provides: {"serde_impls"}
// Dependencies: {}
# [cfg (feature = "serde")] pub mod serde_impls { use serde :: { de :: { Deserialize , Deserializer , Error , Visitor } , ser :: { Serialize , Serializer } , } ; use std :: fmt ; use crate :: Uid ; pub const UID_NEWTYPE_STRUCT_NAME : & str = "PLIST-UID" ; impl Serialize for Uid { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct (UID_NEWTYPE_STRUCT_NAME , & self . get ()) } } struct UidNewtypeVisitor ; impl < 'de > Visitor < 'de > for UidNewtypeVisitor { type Value = Uid ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a plist uid") } fn visit_u64 < E > (self , v : u64) -> Result < Self :: Value , E > where E : Error , { UidU64Visitor . visit_u64 (v) } fn visit_newtype_struct < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_u64 (UidU64Visitor) } } struct UidU64Visitor ; impl Visitor < '_ > for UidU64Visitor { type Value = Uid ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a plist uid") } fn visit_u64 < E > (self , v : u64) -> Result < Self :: Value , E > where E : Error , { Ok (Uid :: new (v)) } } impl < 'de > Deserialize < 'de > for Uid { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_newtype_struct (UID_NEWTYPE_STRUCT_NAME , UidNewtypeVisitor) } } }
};
}
