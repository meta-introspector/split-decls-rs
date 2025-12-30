// Generated macro for midpoint (function)
macro_rules! Depcrate_ir_layoutmidpoint {
() => {
// Module: crate::ir::layout
// Provides: {"midpoint"}
// Dependencies: {}
# [doc = " Compute the midpoint between `a` and `b`."] # [doc = " Return `None` if the midpoint would be equal to either."] fn midpoint (a : SequenceNumber , b : SequenceNumber) -> Option < SequenceNumber > { debug_assert ! (a < b) ; let m = a + (b - a) / 2 ; if m > a { Some (m) } else { None } }
};
}
