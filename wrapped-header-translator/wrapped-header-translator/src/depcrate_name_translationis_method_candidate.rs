// Generated macro for is_method_candidate (function)
macro_rules! Depcrate_name_translationis_method_candidate {
() => {
// Module: crate::name_translation
// Provides: {"is_method_candidate"}
// Dependencies: {}
# [doc = " Whether the function is a candidate for being a method."] fn is_method_candidate (fn_name : & str , type_name : & str) -> bool { let mut fn_words = lowercase_words (fn_name) ; for type_word in lowercase_words (type_name) { if let Some (fn_word) = fn_words . next () { if fn_word == type_word { continue ; } else { return false ; } } else { return false ; } } true }
};
}
