// Generated macro for Operator (struct)
macro_rules! Depcrate_prec_climberOperator {
() => {
// Module: crate::prec_climber
// Provides: {"Operator"}
// Dependencies: {}
# [doc = " Infix operator used in [`PrecClimber`]."] # [doc = ""] # [doc = " [`PrecClimber`]: struct.PrecClimber.html"] # [derive (Debug)] pub struct Operator < R : RuleType > { rule : R , assoc : Assoc , next : Option < Box < Operator < R > > > , }
};
}
