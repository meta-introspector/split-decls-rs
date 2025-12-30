// Generated macro for Job (enum)
macro_rules! Depcrate_backtrackJob {
() => {
// Module: crate::backtrack
// Provides: {"Job"}
// Dependencies: {}
# [doc = " A job is an explicit unit of stack space in the backtracking engine."] # [doc = ""] # [doc = " The \"normal\" representation is a single state transition, which corresponds"] # [doc = " to an NFA state and a character in the input. However, the backtracking"] # [doc = " engine must keep track of old capture group values. We use the explicit"] # [doc = " stack to do it."] # [derive (Clone , Copy , Debug)] enum Job { Inst { ip : InstPtr , at : InputAt } , SaveRestore { slot : usize , old_pos : Option < usize > } , }
};
}
