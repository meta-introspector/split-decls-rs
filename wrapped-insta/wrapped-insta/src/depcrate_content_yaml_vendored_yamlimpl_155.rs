// Generated macro for impl_155 (impl)
macro_rules! Depcrate_content_yaml_vendored_yamlimpl_155 {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"impl_155"}
// Dependencies: {}
impl Yaml { pub fn from_str (v : & str) -> Yaml { if let Some (rest) = v . strip_prefix ("0x") { if let Ok (i) = i64 :: from_str_radix (rest , 16) { return Yaml :: Integer (i) ; } } if let Some (rest) = v . strip_prefix ("0o") { if let Ok (i) = i64 :: from_str_radix (rest , 8) { return Yaml :: Integer (i) ; } } if let Some (rest) = v . strip_prefix ('+') { if let Ok (i) = rest . parse :: < i64 > () { return Yaml :: Integer (i) ; } } match v { "~" | "null" => Yaml :: Null , "true" => Yaml :: Boolean (true) , "false" => Yaml :: Boolean (false) , _ if v . parse :: < i64 > () . is_ok () => Yaml :: Integer (v . parse :: < i64 > () . unwrap ()) , _ if parse_f64 (v) . is_some () => Yaml :: Real (v . to_owned ()) , _ => Yaml :: String (v . to_owned ()) , } } }
};
}
