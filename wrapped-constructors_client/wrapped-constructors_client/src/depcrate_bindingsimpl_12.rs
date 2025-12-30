// Generated macro for impl_12 (impl)
macro_rules! Depcrate_bindingsimpl_12 {
() => {
// Module: crate::bindings
// Provides: {"impl_12"}
// Dependencies: {}
impl Composable { pub fn Property (& self) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Property) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn new () -> windows_core :: Result < Composable > { Self :: IComposableFactory (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . CreateInstance) (windows_core :: Interface :: as_raw (this) , core :: ptr :: null_mut () , & mut core :: ptr :: null_mut () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) }) } pub fn WithValue (arg : i32) -> windows_core :: Result < Composable > { Self :: IComposableFactory (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . WithValue) (windows_core :: Interface :: as_raw (this) , arg , core :: ptr :: null_mut () , & mut core :: ptr :: null_mut () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) }) } fn IComposableFactory < R , F : FnOnce (& IComposableFactory) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Composable , IComposableFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
