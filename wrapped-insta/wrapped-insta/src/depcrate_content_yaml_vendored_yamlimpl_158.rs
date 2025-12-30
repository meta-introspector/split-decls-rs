// Generated macro for impl_158 (impl)
macro_rules! Depcrate_content_yaml_vendored_yamlimpl_158 {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"impl_158"}
// Dependencies: {}
impl Index < usize > for Yaml { type Output = Yaml ; fn index (& self , idx : usize) -> & Yaml { if let Some (v) = self . as_vec () { v . get (idx) . unwrap_or (& BAD_VALUE) } else if let Some (v) = self . as_hash () { let key = Yaml :: Integer (idx as i64) ; v . iter () . find (| x | x . 0 == key) . map_or (& BAD_VALUE , | x | & x . 1) } else { & BAD_VALUE } } }
};
}
