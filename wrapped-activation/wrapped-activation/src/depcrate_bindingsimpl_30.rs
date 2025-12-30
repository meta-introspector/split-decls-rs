// Generated macro for impl_30 (impl)
macro_rules! Depcrate_bindingsimpl_30 {
() => {
// Module: crate::bindings
// Provides: {"impl_30"}
// Dependencies: {}
impl Missing { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Missing , windows_core :: imp :: IGenericFactory , > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn Method (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Method) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
