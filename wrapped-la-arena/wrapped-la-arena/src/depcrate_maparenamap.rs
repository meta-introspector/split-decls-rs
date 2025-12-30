// Generated macro for ArenaMap (struct)
macro_rules! Depcrate_mapArenaMap {
() => {
// Module: crate::map
// Provides: {"ArenaMap"}
// Dependencies: {}
# [doc = " A map from arena indexes to some other type."] # [doc = " Space requirement is O(highest index)."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ArenaMap < IDX , V > { v : Vec < Option < V > > , _ty : PhantomData < IDX > , }
};
}
