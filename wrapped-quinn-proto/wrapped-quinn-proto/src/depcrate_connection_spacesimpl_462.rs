// Generated macro for impl_462 (impl)
macro_rules! Depcrate_connection_spacesimpl_462 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_462"}
// Dependencies: {}
impl Index < SpaceId > for [PacketSpace ; 3] { type Output = PacketSpace ; fn index (& self , space : SpaceId) -> & PacketSpace { & self . as_ref () [space as usize] } }
};
}
