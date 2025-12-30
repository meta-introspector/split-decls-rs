// Generated macro for compute_loan_liveness (function)
macro_rules! Depcrate_polonius_loan_livenesscompute_loan_liveness {
() => {
// Module: crate::polonius::loan_liveness
// Provides: {"compute_loan_liveness"}
// Dependencies: {}
# [doc = " Compute loan reachability to approximately trace loan liveness throughout the CFG, by"] # [doc = " traversing the full graph of constraints that combines:"] # [doc = " - the localized constraints (the physical edges),"] # [doc = " - with the constraints that hold at all points (the logical edges)."] pub (super) fn compute_loan_liveness < 'tcx > (liveness : & LivenessValues , outlives_constraints : impl Iterator < Item = OutlivesConstraint < 'tcx > > , borrow_set : & BorrowSet < 'tcx > , localized_outlives_constraints : & LocalizedOutlivesConstraintSet ,) -> LiveLoans { let mut live_loans = LiveLoans :: new (borrow_set . len ()) ; let logical_constraints = outlives_constraints . filter (| c | matches ! (c . locations , Locations :: All (_))) ; let graph = LocalizedConstraintGraph :: new (& localized_outlives_constraints , logical_constraints) ; let mut visited = FxHashSet :: default () ; let mut stack = Vec :: new () ; for (loan_idx , loan) in borrow_set . iter_enumerated () { visited . clear () ; stack . clear () ; let start_node = LocalizedNode { region : loan . region , point : liveness . point_from_location (loan . reserve_location) , } ; stack . push (start_node) ; while let Some (node) = stack . pop () { if ! visited . insert (node) { continue ; } if liveness . is_live_at (node . region , liveness . location_from_point (node . point)) { live_loans . insert (node . point , loan_idx) ; } for succ in graph . outgoing_edges (node) { stack . push (succ) ; } } } live_loans }
};
}
