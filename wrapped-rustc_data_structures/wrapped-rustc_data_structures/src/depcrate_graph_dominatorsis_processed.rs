// Generated macro for is_processed (function)
macro_rules! Depcrate_graph_dominatorsis_processed {
() => {
// Module: crate::graph::dominators
// Provides: {"is_processed"}
// Dependencies: {}
# [inline] fn is_processed (v : PreorderIndex , lastlinked : Option < PreorderIndex >) -> bool { if let Some (ll) = lastlinked { v >= ll } else { false } }
};
}
