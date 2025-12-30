// Generated macro for impl_197 (impl)
macro_rules! Depcrate_interfaceimpl_197 {
() => {
// Module: crate::interface
// Provides: {"impl_197"}
// Dependencies: {}
impl < I : Interface > core :: ops :: Deref for InterfaceRef < '_ , I > { type Target = I ; # [inline (always)] fn deref (& self) -> & I { unsafe { core :: mem :: transmute (self) } } }
};
}
