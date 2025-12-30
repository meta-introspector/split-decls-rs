// Generated macro for impl_158 (impl)
macro_rules! Depcrateimpl_158 {
() => {
// Module: crate
// Provides: {"impl_158"}
// Dependencies: {}
# [doc = " [`serde::de::Deserialize`] implementation for a [`RelativePath`] reference."] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Document<'a> {"] # [doc = "     #[serde(borrow)]"] # [doc = "     path: &'a RelativePath,"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "serde")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "serde")))] impl < 'de : 'a , 'a > serde :: de :: Deserialize < 'de > for & 'a RelativePath { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor ; impl < 'a > serde :: de :: Visitor < 'a > for Visitor { type Value = & 'a RelativePath ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a borrowed relative path") } # [inline] fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (RelativePath :: new (v)) } # [inline] fn visit_borrowed_bytes < E > (self , v : & 'a [u8]) -> Result < Self :: Value , E > where E : serde :: de :: Error , { let string = str :: from_utf8 (v) . map_err (| _ | { serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Bytes (v) , & self) }) ? ; Ok (RelativePath :: new (string)) } } deserializer . deserialize_str (Visitor) } }
};
}
