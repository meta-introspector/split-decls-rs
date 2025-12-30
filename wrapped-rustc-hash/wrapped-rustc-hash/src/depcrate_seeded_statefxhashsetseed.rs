// Generated macro for FxHashSetSeed (type)
macro_rules! Depcrate_seeded_stateFxHashSetSeed {
() => {
// Module: crate::seeded_state
// Provides: {"FxHashSetSeed"}
// Dependencies: {}
# [doc = " Type alias for a hashmap using the `fx` hash algorithm with [`FxSeededState`]."] # [cfg (feature = "std")] pub type FxHashSetSeed < V > = std :: collections :: HashSet < V , FxSeededState > ;
};
}
