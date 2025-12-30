// Generated macro for impl_178 (impl)
macro_rules! Depcrate_greek_to_meimpl_178 {
() => {
// Module: crate::greek_to_me
// Provides: {"impl_178"}
// Dependencies: {}
impl GreekDiacritics { # [doc = " Whilst forwards-iterating from an existing character,"] # [doc = " consume all further greek diacritics and store their existence into this struct."] pub (crate) fn consume_greek_diacritics (& mut self , context_after : & str) { for c in context_after . chars () { match c { diacritics ! (ACCENTS) => self . accented = true , DIALYTIKA_TONOS => { self . dialytika = true ; self . accented = true ; } DIALYTIKA => self . dialytika = true , YPOGEGRAMMENI => self . ypogegrammeni = true , diacritics ! (BREATHING_AND_LENGTH) => () , _ => break , } } } }
};
}
