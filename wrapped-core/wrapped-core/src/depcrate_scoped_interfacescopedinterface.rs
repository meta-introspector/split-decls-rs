// Generated macro for ScopedInterface (struct)
macro_rules! Depcrate_scoped_interfaceScopedInterface {
() => {
// Module: crate::scoped_interface
// Provides: {"ScopedInterface"}
// Dependencies: {}
# [doc (hidden)] pub struct ScopedInterface < 'a , T : Interface > { interface : T , lifetime : PhantomData < & 'a T > , }
};
}
