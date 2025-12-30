// Generated macro for LoopVisitor (struct)
macro_rules! Depcrate_loops_infinite_loopLoopVisitor {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"LoopVisitor"}
// Dependencies: {}
struct LoopVisitor < 'hir , 'tcx > { cx : & 'hir LateContext < 'tcx > , label : Option < Label > , inner_labels : Vec < Label > , loop_depth : usize , is_finite : bool , }
};
}
