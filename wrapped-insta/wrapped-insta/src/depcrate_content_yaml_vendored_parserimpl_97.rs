// Generated macro for impl_97 (impl)
macro_rules! Depcrate_content_yaml_vendored_parserimpl_97 {
() => {
// Module: crate::content::yaml::vendored::parser
// Provides: {"impl_97"}
// Dependencies: {}
impl Event { fn empty_scalar () -> Event { Event :: Scalar ("~" . to_owned () , TScalarStyle :: Plain , 0 , None) } fn empty_scalar_with_anchor (anchor : usize , tag : Option < TokenType >) -> Event { Event :: Scalar ("" . to_owned () , TScalarStyle :: Plain , anchor , tag) } }
};
}
