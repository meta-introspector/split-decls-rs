// Generated macro for Frame (enum)
macro_rules! Depcrate_nfa_thompson_backtrackFrame {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " Represents a stack frame on the heap while doing backtracking."] # [doc = ""] # [doc = " Instead of using explicit recursion for backtracking, we use a stack on"] # [doc = " the heap to keep track of things that we want to explore if the current"] # [doc = " backtracking branch turns out to not lead to a match."] # [derive (Clone , Debug)] enum Frame { # [doc = " Look for a match starting at `sid` and the given position in the"] # [doc = " haystack."] Step { sid : StateID , at : usize } , # [doc = " Reset the given `slot` to the given `offset` (which might be `None`)."] # [doc = " This effectively gives a \"scope\" to capturing groups, such that an"] # [doc = " offset for a particular group only gets returned if the match goes"] # [doc = " through that capturing group. If backtracking ends up going down a"] # [doc = " different branch that results in a different offset (or perhaps none at"] # [doc = " all), then this \"restore capture\" frame will cause the offset to get"] # [doc = " reset."] RestoreCapture { slot : SmallIndex , offset : Option < NonMaxUsize > } , }
};
}
