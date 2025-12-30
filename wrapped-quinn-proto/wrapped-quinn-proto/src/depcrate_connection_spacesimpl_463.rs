// Generated macro for impl_463 (impl)
macro_rules! Depcrate_connection_spacesimpl_463 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_463"}
// Dependencies: {}
impl IndexMut < SpaceId > for [PacketSpace ; 3] { fn index_mut (& mut self , space : SpaceId) -> & mut PacketSpace { & mut self . as_mut () [space as usize] } }
};
}
