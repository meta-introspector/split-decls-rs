// Generated macro for impl_12 (impl)
macro_rules! Depcrate_bindingsimpl_12 {
() => {
// Module: crate::bindings
// Provides: {"impl_12"}
// Dependencies: {}
impl B { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < B , windows_core :: imp :: IGenericFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn MethodOne (& self) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . MethodOne) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn MethodTwo (& self , a : i32) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . MethodTwo) (windows_core :: Interface :: as_raw (this) , a , & mut result__ ,) . map (| | result__) } } }
};
}
