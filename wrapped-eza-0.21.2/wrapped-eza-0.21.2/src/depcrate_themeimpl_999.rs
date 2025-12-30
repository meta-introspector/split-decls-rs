// Generated macro for impl_999 (impl)
macro_rules! Depcrate_themeimpl_999 {
() => {
// Module: crate::theme
// Provides: {"impl_999"}
// Dependencies: {}
impl ExtensionMappings { fn is_non_empty (& self) -> bool { ! self . mappings . is_empty () } fn add (& mut self , pattern : glob :: Pattern , style : Style) { match (self . mappings . last_mut () , is_simple_pattern (pattern)) { (Some (GlobPattern :: Simple (h)) , Ok (s)) => { h . insert (s , style) ; } (_ , Ok (s)) => { self . mappings . push (GlobPattern :: Simple (HashMap :: from ([(s , style)]))) ; } (_ , Err (p)) => { self . mappings . push (GlobPattern :: Complex (p , style)) ; } } } }
};
}
