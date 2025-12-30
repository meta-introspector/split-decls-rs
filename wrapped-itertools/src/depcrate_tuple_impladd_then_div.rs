// Generated macro for add_then_div (function)
macro_rules! Depcrate_tuple_impladd_then_div {
() => {
// Module: crate::tuple_impl
// Provides: {"add_then_div"}
// Dependencies: {}
# [doc = " `(n + a) / d` avoiding overflow when possible, returns `None` if it overflows."] fn add_then_div (n : usize , a : usize , d : usize) -> Option < usize > { debug_assert_ne ! (d , 0) ; (n / d) . checked_add (a / d) ? . checked_add ((n % d + a % d) / d) }
};
}
