// Generated macro for impl_395 (impl)
macro_rules! Depcrate_algo_astarimpl_395 {
() => {
// Module: crate::algo::astar
// Provides: {"impl_395"}
// Dependencies: {}
impl < G > PathTracker < G > where G : GraphBase , G :: NodeId : Eq + Hash , { fn new () -> PathTracker < G > { PathTracker { came_from : HashMap :: new () , } } fn set_predecessor (& mut self , node : G :: NodeId , previous : G :: NodeId) { self . came_from . insert (node , previous) ; } fn reconstruct_path_to (& self , last : G :: NodeId) -> Vec < G :: NodeId > { let mut path = vec ! [last] ; let mut current = last ; while let Some (& previous) = self . came_from . get (& current) { path . push (previous) ; current = previous ; } path . reverse () ; path } }
};
}
