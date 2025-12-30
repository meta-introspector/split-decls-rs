// Generated macro for impl_628 (impl)
macro_rules! Depcrate_provider_pattern_reference_genericimpl_628 {
() => {
// Module: crate::provider::pattern::reference::generic
// Provides: {"impl_628"}
// Dependencies: {}
impl GenericPattern { # [cfg (test)] pub (crate) fn combined (self , replacements : Vec < Pattern >) -> Result < Pattern , PatternError > { let size = replacements . iter () . fold (0 , | acc , r | acc + r . items . len ()) ; let mut result = Vec :: with_capacity (self . items . len () + size) ; for item in self . items { match item { GenericPatternItem :: Placeholder (idx) => { # [expect (clippy :: unwrap_used)] let replacement = replacements . get (idx as usize) . ok_or_else (| | { PatternError :: UnknownSubstitution (char :: from_digit (idx as u32 , 10) . unwrap ()) }) ? ; result . extend (replacement . items . iter ()) ; } GenericPatternItem :: Literal (ch) => result . push (PatternItem :: Literal (ch)) , } } Ok (result . into ()) } }
};
}
