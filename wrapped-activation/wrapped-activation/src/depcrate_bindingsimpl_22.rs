// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bindingsimpl_22 {
() => {
// Module: crate::bindings
// Provides: {"impl_22"}
// Dependencies: {}
impl Instance { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Instance , windows_core :: imp :: IGenericFactory , > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn Property (& self) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Property) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } }
};
}
