// Generated macro for count_digits_after_dot (function)
macro_rules! Depcrate_approx_constcount_digits_after_dot {
() => {
// Module: crate::approx_const
// Provides: {"count_digits_after_dot"}
// Dependencies: {}
fn count_digits_after_dot (input : & str) -> usize { input . char_indices () . find (| (_ , ch) | * ch == '.') . map_or (0 , | (i , _) | input . len () - i - 1) }
};
}
