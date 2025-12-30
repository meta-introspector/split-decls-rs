// Generated macro for partial_ge (function)
macro_rules! Depcratepartial_ge {
() => {
// Module: crate
// Provides: {"partial_ge"}
// Dependencies: {}
# [doc = " Returns `true` iff `a` and `b` are comparable and `a >= b`."] # [inline] pub fn partial_ge < T : PartialOrd > (a : & T , b : & T) -> bool { a . ge (b) }
};
}
