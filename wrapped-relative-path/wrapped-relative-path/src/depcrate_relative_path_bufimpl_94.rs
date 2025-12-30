// Generated macro for impl_94 (impl)
macro_rules! Depcrate_relative_path_bufimpl_94 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " [`serde::de::Deserialize`] implementation for [`RelativePathBuf`]."] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = " use relative_path::RelativePathBuf;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Document {"] # [doc = "     path: RelativePathBuf,"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "serde")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "serde")))] impl < 'de > serde :: de :: Deserialize < 'de > for RelativePathBuf { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor ; impl serde :: de :: Visitor < '_ > for Visitor { type Value = RelativePathBuf ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a relative path") } # [inline] fn visit_string < E > (self , input : String) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (RelativePathBuf :: from (input)) } # [inline] fn visit_str < E > (self , input : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (RelativePathBuf :: from (input . to_owned ())) } } deserializer . deserialize_str (Visitor) } }
};
}
