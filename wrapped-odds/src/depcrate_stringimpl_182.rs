// Generated macro for impl_182 (impl)
macro_rules! Depcrate_stringimpl_182 {
() => {
// Module: crate::string
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a > CharWindows < 'a > { fn new (s : & 'a str , n : usize) -> Self { assert ! (n != 0) ; match s . char_indices () . nth (n - 1) { None => CharWindows { s : s , a : s . len () , b : s . len () , } , Some ((i , ch)) => CharWindows { s : s , a : 0 , b : i + ch . len_utf8 () , } , } } }
};
}
