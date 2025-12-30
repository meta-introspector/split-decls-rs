// Generated macro for IncrementVisitor (struct)
macro_rules! Depcrate_loops_utilsIncrementVisitor {
() => {
// Module: crate::loops::utils
// Provides: {"IncrementVisitor"}
// Dependencies: {}
# [doc = " Scan a for loop for variables that are incremented exactly once and not used after that."] pub (super) struct IncrementVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , states : HirIdMap < IncrementVisitorVarState > , depth : u32 , }
};
}
