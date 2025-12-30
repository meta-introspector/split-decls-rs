// Generated macro for dfs (function)
macro_rules! Depcrate_acyclicdfs {
() => {
// Module: crate::acyclic
// Provides: {"dfs"}
// Dependencies: {}
# [doc = " Traverse nodes in `graph` in DFS order, starting from `start`, for as long"] # [doc = " as the predicate `valid_order` returns `true` on the current node's order."] fn dfs < G : NodeIndexable + IntoNeighborsDirected + IntoNodeIdentifiers + Visitable > (graph : G , start : G :: NodeId , order_map : & OrderMap < G :: NodeId > , mut valid_order : impl FnMut (TopologicalPosition) -> Result < bool , Cycle < G :: NodeId > > , res : & mut BTreeMap < TopologicalPosition , G :: NodeId > , discovered : & mut FixedBitSet , finished : & mut FixedBitSet ,) -> Result < () , Cycle < G :: NodeId > > where G :: NodeId : IndexType , { dfs_visitor (graph , start , & mut | ev | -> Result < Control < () > , Cycle < G :: NodeId > > { match ev { DfsEvent :: Discover (u , _) => { let order = order_map . get_position (u , & graph) ; res . insert (order , u) ; Ok (Control :: Continue) } DfsEvent :: TreeEdge (_ , u) => { let order = order_map . get_position (u , & graph) ; match valid_order (order) { Ok (true) => Ok (Control :: Continue) , Ok (false) => Ok (Control :: Prune) , Err (cycle) => Err (cycle) , } } _ => Ok (Control :: Continue) , } } , discovered , finished , & mut Time :: default () ,) ? ; Ok (()) }
};
}
