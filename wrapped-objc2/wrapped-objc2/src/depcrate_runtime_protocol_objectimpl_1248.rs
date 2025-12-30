// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_runtime_protocol_objectimpl_1248 {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"impl_1248"}
// Dependencies: {}
impl < P : ? Sized + 'static > AsRef < AnyObject > for ProtocolObject < P > { # [inline] fn as_ref (& self) -> & AnyObject { let ptr : NonNull < ProtocolObject < P > > = NonNull :: from (self) ; let ptr : NonNull < AnyObject > = ptr . cast () ; unsafe { ptr . as_ref () } } }
};
}
