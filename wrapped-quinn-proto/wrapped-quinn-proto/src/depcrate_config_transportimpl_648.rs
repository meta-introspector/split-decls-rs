// Generated macro for impl_648 (impl)
macro_rules! Depcrate_config_transportimpl_648 {
() => {
// Module: crate::config::transport
// Provides: {"impl_648"}
// Dependencies: {}
impl std :: convert :: TryFrom < Duration > for IdleTimeout { type Error = VarIntBoundsExceeded ; fn try_from (timeout : Duration) -> Result < Self , Self :: Error > { let inner = VarInt :: try_from (timeout . as_millis ()) ? ; Ok (Self (inner)) } }
};
}
