// Generated macro for impl_4 (impl)
macro_rules! Depcrate_bindingsimpl_4 {
() => {
// Module: crate::bindings
// Provides: {"impl_4"}
// Dependencies: {}
impl Activatable { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Activatable , windows_core :: imp :: IGenericFactory , > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn Property (& self) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Property) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn WithValue (arg : i32) -> windows_core :: Result < Activatable > { Self :: IActivatableFactory (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . WithValue) (windows_core :: Interface :: as_raw (this) , arg , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) }) } fn IActivatableFactory < R , F : FnOnce (& IActivatableFactory) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Activatable , IActivatableFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
