// Generated macro for branch (function)
macro_rules! Depcrate_rtbranch {
() => {
// Module: crate::rt
// Provides: {"branch"}
// Dependencies: {}
# [doc = " Add an execution branch point."] fn branch < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { let (ret , switch) = execution (| execution | { let ret = f (execution) ; let switch = execution . schedule () ; trace ! (? switch , "branch") ; (ret , switch) }) ; if switch { Scheduler :: switch () ; } ret }
};
}
