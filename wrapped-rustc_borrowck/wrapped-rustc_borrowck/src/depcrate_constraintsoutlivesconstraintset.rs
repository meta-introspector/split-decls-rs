// Generated macro for OutlivesConstraintSet (struct)
macro_rules! Depcrate_constraintsOutlivesConstraintSet {
() => {
// Module: crate::constraints
// Provides: {"OutlivesConstraintSet"}
// Dependencies: {}
# [doc = " A set of NLL region constraints. These include \"outlives\""] # [doc = " constraints of the form `R1: R2`. Each constraint is identified by"] # [doc = " a unique `OutlivesConstraintIndex` and you can index into the set"] # [doc = " (`constraint_set[i]`) to access the constraint details."] # [derive (Clone , Debug , Default)] pub (crate) struct OutlivesConstraintSet < 'tcx > { outlives : IndexVec < OutlivesConstraintIndex , OutlivesConstraint < 'tcx > > , }
};
}
