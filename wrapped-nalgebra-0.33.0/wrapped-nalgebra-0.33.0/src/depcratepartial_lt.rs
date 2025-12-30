// Generated macro for partial_lt (function)
macro_rules! Depcratepartial_lt {
() => {
// Module: crate
// Provides: {"partial_lt"}
// Dependencies: {}
# [doc = " Returns `true` iff `a` and `b` are comparable and `a < b`."] # [inline] pub fn partial_lt < T : PartialOrd > (a : & T , b : & T) -> bool { a . lt (b) }
};
}
