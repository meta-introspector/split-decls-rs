// Generated macro for impl_7139 (impl)
macro_rules! Depcrate_min_ident_charsimpl_7139 {
() => {
// Module: crate::min_ident_chars
// Provides: {"impl_7139"}
// Dependencies: {}
impl MinIdentChars { pub fn new (conf : & 'static Conf) -> Self { Self { allowed_idents_below_min_chars : conf . allowed_idents_below_min_chars . iter () . cloned () . collect () , min_ident_chars_threshold : conf . min_ident_chars_threshold , } } # [expect (clippy :: cast_possible_truncation)] fn is_ident_too_short (& self , cx : & LateContext < '_ > , str : & str , span : Span) -> bool { ! span . in_external_macro (cx . sess () . source_map ()) && str . len () <= self . min_ident_chars_threshold as usize && ! str . starts_with ('_') && ! str . is_empty () && ! self . allowed_idents_below_min_chars . contains (str) } }
};
}
