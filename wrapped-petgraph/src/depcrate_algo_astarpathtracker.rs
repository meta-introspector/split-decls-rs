// Generated macro for PathTracker (struct)
macro_rules! Depcrate_algo_astarPathTracker {
() => {
// Module: crate::algo::astar
// Provides: {"PathTracker"}
// Dependencies: {}
struct PathTracker < G > where G : GraphBase , G :: NodeId : Eq + Hash , { came_from : HashMap < G :: NodeId , G :: NodeId > , }
};
}
