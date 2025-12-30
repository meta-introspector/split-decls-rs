// Generated macro for impl_4 (impl)
macro_rules! Depcrate_bindingsimpl_4 {
() => {
// Module: crate::bindings
// Provides: {"impl_4"}
// Dependencies: {}
impl A { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < A , windows_core :: imp :: IGenericFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn Method (& self) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Method) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn Method2 (& self , a : i32) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Method2) (windows_core :: Interface :: as_raw (this) , a , & mut result__ ,) . map (| | result__) } } }
};
}
