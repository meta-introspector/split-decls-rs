// Generated macro for Fsm (struct)
macro_rules! Depcrate_pikevmFsm {
() => {
// Module: crate::pikevm
// Provides: {"Fsm"}
// Dependencies: {}
# [doc = " An NFA simulation matching engine."] # [derive (Debug)] pub struct Fsm < 'r , I > { # [doc = " The sequence of opcodes (among other things) that is actually executed."] # [doc = ""] # [doc = " The program may be byte oriented or Unicode codepoint oriented."] prog : & 'r Program , # [doc = " An explicit stack used for following epsilon transitions. (This is"] # [doc = " borrowed from the cache.)"] stack : & 'r mut Vec < FollowEpsilon > , # [doc = " The input to search."] input : I , }
};
}
