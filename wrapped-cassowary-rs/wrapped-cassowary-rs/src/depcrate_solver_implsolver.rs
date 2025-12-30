// Generated macro for Solver (struct)
macro_rules! Depcrate_solver_implSolver {
() => {
// Module: crate::solver_impl
// Provides: {"Solver"}
// Dependencies: {}
# [doc = " A constraint solver using the Cassowary algorithm. For proper usage please see the top level crate documentation."] pub struct Solver { cns : HashMap < Constraint , Tag > , var_data : HashMap < Variable , (f64 , Symbol , usize) > , var_for_symbol : HashMap < Symbol , Variable > , public_changes : Vec < (Variable , f64) > , changed : HashSet < Variable > , should_clear_changes : bool , rows : HashMap < Symbol , Box < Row > > , edits : HashMap < Variable , EditInfo > , infeasible_rows : Vec < Symbol > , objective : Rc < RefCell < Row > > , artificial : Option < Rc < RefCell < Row > > > , id_tick : usize }
};
}
