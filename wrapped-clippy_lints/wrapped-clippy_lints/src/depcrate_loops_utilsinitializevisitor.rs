// Generated macro for InitializeVisitor (struct)
macro_rules! Depcrate_loops_utilsInitializeVisitor {
() => {
// Module: crate::loops::utils
// Provides: {"InitializeVisitor"}
// Dependencies: {}
# [doc = " Checks whether a variable is initialized at the start of a loop and not modified"] # [doc = " and used after the loop."] pub (super) struct InitializeVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , end_expr : & 'tcx Expr < 'tcx > , var_id : HirId , state : InitializeVisitorState < 'tcx > , depth : u32 , past_loop : bool , }
};
}
