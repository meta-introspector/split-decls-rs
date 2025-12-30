// Generated macro for to_string (function)
macro_rules! Depcrate_content_yamlto_string {
() => {
// Module: crate::content::yaml
// Provides: {"to_string"}
// Dependencies: {}
pub fn to_string (content : & Content) -> String { let yaml_blob = to_yaml_value (content) ; let mut buf = String :: new () ; let mut emitter = crate :: content :: yaml :: vendored :: emitter :: YamlEmitter :: new (& mut buf) ; emitter . dump (& yaml_blob) . unwrap () ; if ! buf . ends_with ('\n') { buf . push ('\n') ; } buf }
};
}
