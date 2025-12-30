// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_runtime_protocol_objectimpl_1246 {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"impl_1246"}
// Dependencies: {}
impl < P : ? Sized + NSObjectProtocol > fmt :: Debug for ProtocolObject < P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let description = self . description () ; autoreleasepool_leaking (| pool | { let s = unsafe { nsstring_to_str (& description , pool) } ; fmt :: Display :: fmt (s , f) }) } }
};
}
