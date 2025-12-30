// Generated macro for impl_1079 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1079 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1079"}
// Dependencies: {}
impl InternalTransliterator { fn transliterate (& self , mut rep : Replaceable , env : & Env) { match self { Self :: RuleBased (rbt) => rbt . get () . transliterate (rep , env) , Self :: Composing (normalizer) => { if let Cow :: Owned (buf) = normalizer . as_borrowed () . normalize (rep . as_str_modifiable ()) { rep . replace_modifiable_with_str (& buf) ; } } Self :: Decomposing (normalizer) => { if let Cow :: Owned (buf) = normalizer . as_borrowed () . normalize (rep . as_str_modifiable ()) { rep . replace_modifiable_with_str (& buf) ; } } Self :: Lower (casemap) => { if let Cow :: Owned (buf) = casemap . as_borrowed () . lowercase_to_string (rep . as_str_modifiable () , & LanguageIdentifier :: UNKNOWN) { rep . replace_modifiable_with_str (& buf) ; } } Self :: Upper (casemap) => { if let Cow :: Owned (buf) = casemap . as_borrowed () . uppercase_to_string (rep . as_str_modifiable () , & LanguageIdentifier :: UNKNOWN) { rep . replace_modifiable_with_str (& buf) ; } } Self :: Hex (t) => t . transliterate (rep) , Self :: Null => () , Self :: Remove => rep . replace_modifiable_with_str ("") , Self :: Dyn (custom) => { let replacement = custom . transliterate (rep . as_str () , rep . allowed_range ()) ; rep . replace_modifiable_with_str (& replacement) } } } }
};
}
