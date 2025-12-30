// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
# [doc = " [`serde::ser::Serialize`] implementation for [`RelativePath`]."] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Serialize;"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Document<'a> {"] # [doc = "     path: &'a RelativePath,"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "serde")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "serde")))] impl serde :: ser :: Serialize for RelativePath { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { serializer . serialize_str (& self . inner) } }
};
}
