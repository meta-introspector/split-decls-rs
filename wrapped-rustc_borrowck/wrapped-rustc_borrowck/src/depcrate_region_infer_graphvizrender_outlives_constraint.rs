// Generated macro for render_outlives_constraint (function)
macro_rules! Depcrate_region_infer_graphvizrender_outlives_constraint {
() => {
// Module: crate::region_infer::graphviz
// Provides: {"render_outlives_constraint"}
// Dependencies: {}
fn render_outlives_constraint (constraint : & OutlivesConstraint < '_ >) -> String { if let ConstraintCategory :: OutlivesUnnameablePlaceholder (unnameable) = constraint . category { format ! ("{unnameable:?} unnameable") } else { match constraint . locations { Locations :: All (_) => "All(...)" . to_string () , Locations :: Single (loc) => format ! ("{loc:?}") , } } }
};
}
