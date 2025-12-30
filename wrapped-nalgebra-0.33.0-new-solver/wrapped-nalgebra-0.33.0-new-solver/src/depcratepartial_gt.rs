// Generated macro for partial_gt (function)
macro_rules! Depcratepartial_gt {
() => {
// Module: crate
// Provides: {"partial_gt"}
// Dependencies: {}
# [doc = " Returns `true` iff `a` and `b` are comparable and `a > b`."] # [inline] pub fn partial_gt < T : PartialOrd > (a : & T , b : & T) -> bool { a . gt (b) }
};
}
