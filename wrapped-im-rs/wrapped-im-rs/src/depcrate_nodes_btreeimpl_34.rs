// Generated macro for impl_34 (impl)
macro_rules! Depcrate_nodes_btreeimpl_34 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_34"}
// Dependencies: {}
impl < A > Node < A > { # [inline] fn has_room (& self) -> bool { self . keys . len () < NODE_SIZE } # [inline] fn too_small (& self) -> bool { self . keys . len () < MEDIAN } # [inline] pub (crate) fn unit (value : A) -> Self { Node { keys : Chunk :: unit (value) , children : Chunk :: pair (None , None) , } } # [inline] pub (crate) fn new_from_split (pool : & Pool < Node < A > > , left : Node < A > , median : A , right : Node < A > ,) -> Self { Node { keys : Chunk :: unit (median) , children : Chunk :: pair (Some (PoolRef :: new (pool , left)) , Some (PoolRef :: new (pool , right)) ,) , } } pub (crate) fn min (& self) -> Option < & A > { match self . children . first () . unwrap () { None => self . keys . first () , Some (ref child) => child . min () , } } pub (crate) fn max (& self) -> Option < & A > { match self . children . last () . unwrap () { None => self . keys . last () , Some (ref child) => child . max () , } } }
};
}
