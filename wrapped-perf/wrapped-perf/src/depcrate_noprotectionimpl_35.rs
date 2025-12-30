// Generated macro for impl_35 (impl)
macro_rules! Depcrate_noprotectionimpl_35 {
() => {
// Module: crate::noprotection
// Provides: {"impl_35"}
// Dependencies: {}
impl NoProtectionSession { fn new (tls : Box < dyn crypto :: Session >) -> Self { Self { inner : tls } } # [doc = " Wraps the provided keys in `NoProtectionPacketKey` to disable packet encryption / decryption"] fn wrap_packet_keys (keys : crypto :: KeyPair < Box < dyn crypto :: PacketKey > > ,) -> crypto :: KeyPair < Box < dyn crypto :: PacketKey > > { crypto :: KeyPair { local : Box :: new (NoProtectionPacketKey :: new (keys . local)) , remote : Box :: new (NoProtectionPacketKey :: new (keys . remote)) , } } }
};
}
