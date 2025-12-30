// Generated macro for impl_243 (impl)
macro_rules! Depcrate_topo_placerimpl_243 {
() => {
// Module: crate::topo::placer
// Provides: {"impl_243"}
// Dependencies: {}
impl BlockKind { pub fn is_box (& self) -> bool { match self { BlockKind :: None | BlockKind :: Connector => false , BlockKind :: Both | BlockKind :: Box => true , } } pub fn is_connector (& self) -> bool { match self { BlockKind :: None | BlockKind :: Box => false , BlockKind :: Both | BlockKind :: Connector => true , } } }
};
}
