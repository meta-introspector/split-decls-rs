// Generated macro for impl_121 (impl)
macro_rules! Depcrate_content_yaml_vendored_scannerimpl_121 {
() => {
// Module: crate::content::yaml::vendored::scanner
// Provides: {"impl_121"}
// Dependencies: {}
impl < T : Iterator < Item = char > > Iterator for Scanner < T > { type Item = Token ; fn next (& mut self) -> Option < Token > { if self . error . is_some () { return None ; } match self . next_token () { Ok (tok) => tok , Err (e) => { self . error = Some (e) ; None } } } }
};
}
