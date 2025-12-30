// Generated macro for impl_196 (impl)
macro_rules! Depcrate_interfaceimpl_196 {
() => {
// Module: crate::interface
// Provides: {"impl_196"}
// Dependencies: {}
impl < 'a , 'i : 'a , I : Interface > From < & 'i I > for InterfaceRef < 'a , I > { # [inline (always)] fn from (interface : & 'a I) -> Self { InterfaceRef :: from_interface (interface) } }
};
}
