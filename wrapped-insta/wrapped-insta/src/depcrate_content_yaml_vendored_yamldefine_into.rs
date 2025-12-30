// Generated macro for define_into (macro)
macro_rules! Depcrate_content_yaml_vendored_yamldefine_into {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"define_into"}
// Dependencies: {}
macro_rules ! define_into (($ name : ident , $ t : ty , $ yt : ident) => (pub fn $ name (self) -> Option <$ t > { match self { Yaml ::$ yt (v) => Some (v) , _ => None } }) ;) ;
};
}
