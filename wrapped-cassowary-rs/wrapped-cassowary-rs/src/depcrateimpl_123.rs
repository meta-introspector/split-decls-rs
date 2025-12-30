// Generated macro for impl_123 (impl)
macro_rules! Depcrateimpl_123 {
() => {
// Module: crate
// Provides: {"impl_123"}
// Dependencies: {}
impl Constraint { # [doc = " Construct a new constraint from an expression, a relational operator and a strength."] # [doc = " This corresponds to the equation `e op 0.0`, e.g. `x + y >= 0.0`. For equations with a non-zero"] # [doc = " right hand side, subtract it from the equation to give a zero right hand side."] pub fn new (e : Expression , op : RelationalOperator , strength : f64) -> Constraint { Constraint (Arc :: new (ConstraintData { expression : e , op : op , strength : strength })) } # [doc = " The expression of the left hand side of the constraint equation."] pub fn expr (& self) -> & Expression { & self . 0 . expression } # [doc = " The relational operator governing the constraint."] pub fn op (& self) -> RelationalOperator { self . 0 . op } # [doc = " The strength of the constraint that the solver will use."] pub fn strength (& self) -> f64 { self . 0 . strength } }
};
}
