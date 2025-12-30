// Generated macro for impl_93 (impl)
macro_rules! Depcrate_relative_path_bufimpl_93 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_93"}
// Dependencies: {}
# [doc = " [`serde::ser::Serialize`] implementation for [`RelativePathBuf`]."] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Serialize;"] # [doc = " use relative_path::RelativePathBuf;"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Document {"] # [doc = "     path: RelativePathBuf,"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "serde")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "serde")))] impl serde :: ser :: Serialize for RelativePathBuf { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { serializer . serialize_str (& self . inner) } }
};
}
