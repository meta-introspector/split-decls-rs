// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_runtime_protocol_objectimpl_1247 {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"impl_1247"}
// Dependencies: {}
impl < P : ? Sized , T > AsRef < ProtocolObject < T > > for ProtocolObject < P > where T : ? Sized + ImplementedBy < ProtocolObject < P > > , { # [inline] fn as_ref (& self) -> & ProtocolObject < T > { ProtocolObject :: from_ref (self) } }
};
}
