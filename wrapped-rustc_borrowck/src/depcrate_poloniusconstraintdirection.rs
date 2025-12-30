// Generated macro for ConstraintDirection (enum)
macro_rules! Depcrate_poloniusConstraintDirection {
() => {
// Module: crate::polonius
// Provides: {"ConstraintDirection"}
// Dependencies: {}
# [doc = " The direction a constraint can flow into. Used to create liveness constraints according to"] # [doc = " variance."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ConstraintDirection { # [doc = " For covariant cases, we add a forward edge `O at P1 -> O at P2`."] Forward , # [doc = " For contravariant cases, we add a backward edge `O at P2 -> O at P1`"] Backward , # [doc = " For invariant cases, we add both the forward and backward edges `O at P1 <-> O at P2`."] Bidirectional , }
};
}
