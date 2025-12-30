// Generated macro for DotCrateGraph (struct)
macro_rules! Depcrate_view_crate_graphDotCrateGraph {
() => {
// Module: crate::view_crate_graph
// Provides: {"DotCrateGraph"}
// Dependencies: {}
struct DotCrateGraph < 'db > { crates_to_render : FxHashMap < Crate , (& 'db BuiltCrateData , & 'db ExtraCrateData) > , }
};
}
