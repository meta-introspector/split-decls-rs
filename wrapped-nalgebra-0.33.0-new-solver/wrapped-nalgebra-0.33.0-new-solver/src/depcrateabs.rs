// Generated macro for abs (function)
macro_rules! Depcrateabs {
() => {
// Module: crate
// Provides: {"abs"}
// Dependencies: {}
# [doc = " The absolute value of `a`."] # [doc = ""] # [doc = " Deprecated: Use [`Matrix::abs()`] or [`ComplexField::abs()`] instead."] # [deprecated (note = "use the inherent method `Matrix::abs` or `ComplexField::abs` instead")] # [inline] pub fn abs < T : Signed > (a : & T) -> T { a . abs () }
};
}
