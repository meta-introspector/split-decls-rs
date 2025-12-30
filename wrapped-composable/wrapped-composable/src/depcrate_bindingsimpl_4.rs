// Generated macro for impl_4 (impl)
macro_rules! Depcrate_bindingsimpl_4 {
() => {
// Module: crate::bindings
// Provides: {"impl_4"}
// Dependencies: {}
impl Compositor { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Compositor , windows_core :: imp :: IGenericFactory , > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn CreateSpriteVisual (& self , brush : i32) -> windows_core :: Result < SpriteVisual > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . CreateSpriteVisual) (windows_core :: Interface :: as_raw (this) , brush , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn CreateContainerVisual (& self , children : i32) -> windows_core :: Result < ContainerVisual > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . CreateContainerVisual) (windows_core :: Interface :: as_raw (this) , children , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
