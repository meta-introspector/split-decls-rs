// Generated macro for impl_266 (impl)
macro_rules! Depcrate_validation_rules_known_argument_namesimpl_266 {
() => {
// Module: crate::validation::rules::known_argument_names
// Provides: {"impl_266"}
// Dependencies: {}
impl KnownArgumentNames < '_ > { fn get_suggestion (& self , name : & str) -> String { make_suggestion (" Did you mean" , self . current_args . iter () . map (| (args , _) | args . iter () . map (| arg | arg . 0 . as_str ())) . flatten () , name ,) . unwrap_or_default () } }
};
}
