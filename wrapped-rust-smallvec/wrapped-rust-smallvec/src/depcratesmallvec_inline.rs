// Generated macro for smallvec_inline (macro)
macro_rules! Depcratesmallvec_inline {
() => {
// Module: crate
// Provides: {"smallvec_inline"}
// Dependencies: {}
# [macro_export] macro_rules ! smallvec_inline { (@ one $ x : expr) => (1usize) ; ($ elem : expr ; $ n : expr) => ({ $ crate :: SmallVec ::< _ , $ n >:: from_buf ([$ elem ; $ n]) }) ; ($ ($ x : expr) ,+ $ (,) ?) => ({ const N : usize = 0usize $ (+ $ crate :: smallvec_inline ! (@ one $ x)) *; $ crate :: SmallVec ::< _ , N >:: from_buf ([$ ($ x ,) *]) }) ; }
};
}
