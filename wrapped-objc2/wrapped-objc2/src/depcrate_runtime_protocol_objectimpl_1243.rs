// Generated macro for impl_1243 (impl)
macro_rules! Depcrate_runtime_protocol_objectimpl_1243 {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"impl_1243"}
// Dependencies: {}
impl < P : ? Sized + NSObjectProtocol > PartialEq for ProtocolObject < P > { # [inline] # [doc (alias = "isEqual:")] fn eq (& self , other : & Self) -> bool { self . isEqual (Some (& other . inner)) } }
};
}
