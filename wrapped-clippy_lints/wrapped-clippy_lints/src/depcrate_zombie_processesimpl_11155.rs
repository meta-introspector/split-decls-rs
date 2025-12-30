// Generated macro for impl_11155 (impl)
macro_rules! Depcrate_zombie_processesimpl_11155 {
() => {
// Module: crate::zombie_processes
// Provides: {"impl_11155"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ExitPointFinder < '_ , 'tcx > { type Result = ControlFlow < ExitCallFound > ; fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> Self :: Result { match self . state { ExitPointState :: WalkUpTo (id) if expr . hir_id == id => { self . state = ExitPointState :: NoExit ; walk_expr (self , expr) } , ExitPointState :: NoExit if expr_enters_control_flow (expr) => { self . state = ExitPointState :: InControlFlow { depth : 1 } ; walk_expr (self , expr) ? ; if let ExitPointState :: InControlFlow { .. } = self . state { self . state = ExitPointState :: NoExit ; } Continue (()) } , ExitPointState :: NoExit if is_exit_expression (self . cx , expr) => Break (ExitCallFound) , ExitPointState :: InControlFlow { ref mut depth } if expr_enters_control_flow (expr) => { * depth += 1 ; walk_expr (self , expr) ? ; match self . state { ExitPointState :: InControlFlow { depth : 1 } => self . state = ExitPointState :: NoExit , ExitPointState :: InControlFlow { ref mut depth } => * depth -= 1 , _ => { } , } Continue (()) } , _ => Continue (()) , } } }
};
}
