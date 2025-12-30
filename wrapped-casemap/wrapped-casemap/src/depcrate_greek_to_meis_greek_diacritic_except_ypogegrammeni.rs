// Generated macro for is_greek_diacritic_except_ypogegrammeni (function)
macro_rules! Depcrate_greek_to_meis_greek_diacritic_except_ypogegrammeni {
() => {
// Module: crate::greek_to_me
// Provides: {"is_greek_diacritic_except_ypogegrammeni"}
// Dependencies: {}
# [doc = " Is the character a diacritic expected to be used with greek (except ypogegrammeni)."] pub (crate) fn is_greek_diacritic_except_ypogegrammeni (c : char) -> bool { matches ! (c , diacritics ! (ACCENTS | BREATHING_AND_LENGTH | DIALYTIKA_ALL)) }
};
}
