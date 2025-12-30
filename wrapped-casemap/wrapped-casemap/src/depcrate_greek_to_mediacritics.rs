// Generated macro for diacritics (macro)
macro_rules! Depcrate_greek_to_mediacritics {
() => {
// Module: crate::greek_to_me
// Provides: {"diacritics"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! diacritics { (ACCENTS) => { '\u{0300}' | $ crate :: greek_to_me :: TONOS | '\u{0342}' | '\u{0302}' | '\u{0303}' | '\u{0311}' } ; (BREATHING_AND_LENGTH) => { '\u{0304}' | '\u{0306}' | '\u{0313}' | '\u{0314}' | '\u{0343}' } ; (DIALYTIKA_ALL) => { $ crate :: greek_to_me :: DIALYTIKA | $ crate :: greek_to_me :: DIALYTIKA_TONOS } ; (DIALYTIKA) => { $ crate :: greek_to_me :: DIALYTIKA } ; (DIALYTIKA_TONOS) => { $ crate :: greek_to_me :: DIALYTIKA_TONOS } ; (YPOGEGRAMMENI) => { $ crate :: greek_to_me :: YPOGEGRAMMENI } ; ($ ($ i : ident) |+) => { $ (diacritics ! ($ i)) |+ } ; }
};
}
