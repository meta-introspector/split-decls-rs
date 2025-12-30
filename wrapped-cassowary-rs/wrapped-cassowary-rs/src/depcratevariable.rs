// Generated macro for Variable (struct)
macro_rules! DepcrateVariable {
() => {
// Module: crate
// Provides: {"Variable"}
// Dependencies: {}
# [doc = " Identifies a variable for the constraint solver."] # [doc = " Each new variable is unique in the view of the solver, but copying or cloning the variable produces"] # [doc = " a copy of the same variable."] # [derive (Copy , Clone , Hash , PartialEq , Eq , PartialOrd , Ord , Debug)] pub struct Variable (usize) ;
};
}
