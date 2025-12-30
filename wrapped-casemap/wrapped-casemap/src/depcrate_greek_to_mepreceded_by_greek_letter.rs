// Generated macro for preceded_by_greek_letter (function)
macro_rules! Depcrate_greek_to_mepreceded_by_greek_letter {
() => {
// Module: crate::greek_to_me
// Provides: {"preceded_by_greek_letter"}
// Dependencies: {}
# [doc = " Given the context before a character, check if it is preceded by a Greek letter."] pub (crate) fn preceded_by_greek_letter (context_before : & str) -> bool { for c in context_before . chars () . rev () { match c { diacritics ! (ACCENTS | BREATHING_AND_LENGTH | DIALYTIKA_ALL | YPOGEGRAMMENI) => continue , _ => return get_data (c) . is_some () , } } false }
};
}
