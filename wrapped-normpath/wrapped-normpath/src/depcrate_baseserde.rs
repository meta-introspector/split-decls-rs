// Generated macro for serde (module)
macro_rules! Depcrate_baseserde {
() => {
// Module: crate::base
// Provides: {"serde"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg_attr (normpath_docs_rs , doc (cfg (feature = "serde")))] mod serde { use std :: ffi :: OsString ; use serde :: Deserialize ; use serde :: Deserializer ; use serde :: Serialize ; use serde :: Serializer ; use super :: BasePath ; use super :: BasePathBuf ; impl < 'de > Deserialize < 'de > for BasePathBuf { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { OsString :: deserialize (deserializer) . map (| x | Self (x . into ())) } } impl Serialize for BasePath { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct ("BasePath" , & self . 0) } } impl Serialize for BasePathBuf { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct ("BasePathBuf" , self . as_os_str ()) } } }
};
}
