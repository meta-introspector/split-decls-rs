// Generated macro for abs_sub (function)
macro_rules! Depcrate_signabs_sub {
() => {
// Module: crate::sign
// Provides: {"abs_sub"}
// Dependencies: {}
# [doc = " The positive difference of two numbers."] # [doc = ""] # [doc = " Returns zero if `x` is less than or equal to `y`, otherwise the difference"] # [doc = " between `x` and `y` is returned."] # [inline (always)] pub fn abs_sub < T : Signed > (x : T , y : T) -> T { x . abs_sub (& y) }
};
}
