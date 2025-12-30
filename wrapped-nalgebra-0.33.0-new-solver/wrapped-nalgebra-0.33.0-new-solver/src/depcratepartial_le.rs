// Generated macro for partial_le (function)
macro_rules! Depcratepartial_le {
() => {
// Module: crate
// Provides: {"partial_le"}
// Dependencies: {}
# [doc = " Returns `true` iff `a` and `b` are comparable and `a <= b`."] # [inline] pub fn partial_le < T : PartialOrd > (a : & T , b : & T) -> bool { a . le (b) }
};
}
