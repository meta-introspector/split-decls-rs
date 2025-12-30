// Generated macro for impl_157 (impl)
macro_rules! Depcrate_content_yaml_vendored_yamlimpl_157 {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a > Index < & 'a str > for Yaml { type Output = Yaml ; fn index (& self , idx : & 'a str) -> & Yaml { let key = Yaml :: String (idx . to_owned ()) ; match self . as_hash () { Some (h) => h . iter () . find (| x | x . 0 == key) . map_or (& BAD_VALUE , | x | & x . 1) , None => & BAD_VALUE , } } }
};
}
