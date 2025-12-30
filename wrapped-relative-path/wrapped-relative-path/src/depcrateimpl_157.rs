// Generated macro for impl_157 (impl)
macro_rules! Depcrateimpl_157 {
() => {
// Module: crate
// Provides: {"impl_157"}
// Dependencies: {}
# [doc = " [`serde::de::Deserialize`] implementation for [`Box<RelativePath>`]."] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Document {"] # [doc = "     path: Box<RelativePath>,"] # [doc = " }"] # [doc = " ```"] # [cfg (all (feature = "alloc" , feature = "serde"))] # [cfg_attr (relative_path_docsrs , doc (cfg (all (feature = "alloc" , feature = "serde"))))] impl < 'de > serde :: de :: Deserialize < 'de > for Box < RelativePath > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor ; impl serde :: de :: Visitor < '_ > for Visitor { type Value = Box < RelativePath > ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a relative path") } # [cfg (feature = "alloc")] # [inline] fn visit_string < E > (self , input : String) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (Box :: < RelativePath > :: from (input . into_boxed_str ())) } # [inline] fn visit_str < E > (self , input : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (Box :: < RelativePath > :: from (input)) } } deserializer . deserialize_str (Visitor) } }
};
}
