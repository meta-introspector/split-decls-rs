// Generated macro for wrap (function)
macro_rules! Depcratewrap {
() => {
// Module: crate
// Provides: {"wrap"}
// Dependencies: {}
# [doc = " Wraps `val` into the range `[min, max]` using modular arithmetics."] # [doc = ""] # [doc = " The range must not be empty."] # [must_use] # [inline] pub fn wrap < T > (mut val : T , min : T , max : T) -> T where T : Copy + PartialOrd + ClosedAddAssign + ClosedSubAssign , { assert ! (min < max , "Invalid wrapping bounds.") ; let width = max - min ; if val < min { val += width ; while val < min { val += width } } else if val > max { val -= width ; while val > max { val -= width } } val }
};
}
