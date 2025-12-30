// Generated macro for define_as (macro)
macro_rules! Depcrate_content_yaml_vendored_yamldefine_as {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"define_as"}
// Dependencies: {}
macro_rules ! define_as (($ name : ident , $ t : ident , $ yt : ident) => (pub fn $ name (& self) -> Option <$ t > { match * self { Yaml ::$ yt (v) => Some (v) , _ => None } }) ;) ;
};
}
