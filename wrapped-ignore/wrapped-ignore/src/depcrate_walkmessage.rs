// Generated macro for Message (enum)
macro_rules! Depcrate_walkMessage {
() => {
// Module: crate::walk
// Provides: {"Message"}
// Dependencies: {}
# [doc = " Message is the set of instructions that a worker knows how to process."] enum Message { # [doc = " A work item corresponds to a directory that should be descended into."] # [doc = " Work items for entries that should be skipped or ignored should not"] # [doc = " be produced."] Work (Work) , # [doc = " This instruction indicates that the worker should quit."] Quit , }
};
}
