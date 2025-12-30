// Generated macro for impl_159 (impl)
macro_rules! Depcrate_content_yaml_vendored_yamlimpl_159 {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"impl_159"}
// Dependencies: {}
impl IntoIterator for Yaml { type Item = Yaml ; type IntoIter = YamlIter ; fn into_iter (self) -> Self :: IntoIter { YamlIter { yaml : self . into_vec () . unwrap_or_default () . into_iter () , } } }
};
}
