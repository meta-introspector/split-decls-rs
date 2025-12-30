// Generated macro for tuple_len (macro)
macro_rules! Depcrate_utils_tupletuple_len {
() => {
// Module: crate::utils::tuple
// Provides: {"tuple_len"}
// Dependencies: {}
# [doc = " Calculate the number of tuples currently being operated on."] macro_rules ! tuple_len { (@ count_one $ F : ident) => (1) ; ($ ($ F : ident ,) *) => (0 $ (+ crate :: utils :: tuple_len ! (@ count_one $ F)) *) ; }
};
}
