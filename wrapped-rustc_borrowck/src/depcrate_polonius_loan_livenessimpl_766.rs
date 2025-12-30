// Generated macro for impl_766 (impl)
macro_rules! Depcrate_polonius_loan_livenessimpl_766 {
() => {
// Module: crate::polonius::loan_liveness
// Provides: {"impl_766"}
// Dependencies: {}
impl LocalizedConstraintGraph { # [doc = " Traverses the constraints and returns the indexed graph of edges per node."] fn new < 'tcx > (constraints : & LocalizedOutlivesConstraintSet , logical_constraints : impl Iterator < Item = OutlivesConstraint < 'tcx > > ,) -> Self { let mut edges : FxHashMap < _ , FxIndexSet < _ > > = FxHashMap :: default () ; for constraint in & constraints . outlives { let source = LocalizedNode { region : constraint . source , point : constraint . from } ; let target = LocalizedNode { region : constraint . target , point : constraint . to } ; edges . entry (source) . or_default () . insert (target) ; } let mut logical_edges : FxHashMap < _ , FxIndexSet < _ > > = FxHashMap :: default () ; for constraint in logical_constraints { logical_edges . entry (constraint . sup) . or_default () . insert (constraint . sub) ; } LocalizedConstraintGraph { edges , logical_edges } } # [doc = " Returns the outgoing edges of a given node, not its transitive closure."] fn outgoing_edges (& self , node : LocalizedNode) -> impl Iterator < Item = LocalizedNode > { let physical_edges = self . edges . get (& node) . into_iter () . flat_map (| targets | targets . iter () . copied ()) ; let materialized_edges = self . logical_edges . get (& node . region) . into_iter () . flat_map (move | targets | { targets . iter () . copied () . map (move | target | LocalizedNode { point : node . point , region : target }) }) ; physical_edges . chain (materialized_edges) } }
};
}
