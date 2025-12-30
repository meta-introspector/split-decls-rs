// Generated macro for define_as_ref (macro)
macro_rules! Depcrate_content_yaml_vendored_yamldefine_as_ref {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"define_as_ref"}
// Dependencies: {}
macro_rules ! define_as_ref (($ name : ident , $ t : ty , $ yt : ident) => (pub fn $ name (& self) -> Option <$ t > { match * self { Yaml ::$ yt (ref v) => Some (v) , _ => None } }) ;) ;
};
}
