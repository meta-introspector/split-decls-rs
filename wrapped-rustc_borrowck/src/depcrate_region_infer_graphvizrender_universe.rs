// Generated macro for render_universe (function)
macro_rules! Depcrate_region_infer_graphvizrender_universe {
() => {
// Module: crate::region_infer::graphviz
// Provides: {"render_universe"}
// Dependencies: {}
fn render_universe (u : UniverseIndex) -> String { if u . is_root () { return "" . to_string () ; } format ! ("/{:?}" , u) }
};
}
