// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_runtime_protocol_objectimpl_1245 {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"impl_1245"}
// Dependencies: {}
impl < P : ? Sized + NSObjectProtocol > hash :: Hash for ProtocolObject < P > { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { < Self as NSObjectProtocol > :: hash (self) . hash (state) ; } }
};
}
