// Generated macro for impl_242 (impl)
macro_rules! Depcrate_scoped_interfaceimpl_242 {
() => {
// Module: crate::scoped_interface
// Provides: {"impl_242"}
// Dependencies: {}
impl < T : Interface > ScopedInterface < '_ , T > { pub fn new (interface : T) -> Self { Self { interface , lifetime : PhantomData , } } }
};
}
