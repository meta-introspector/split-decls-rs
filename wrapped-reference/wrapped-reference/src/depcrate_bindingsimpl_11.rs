// Generated macro for impl_11 (impl)
macro_rules! Depcrate_bindingsimpl_11 {
() => {
// Module: crate::bindings
// Provides: {"impl_11"}
// Dependencies: {}
impl Reference { pub fn new () -> windows_core :: Result < Self > { Self :: IActivationFactory (| f | f . ActivateInstance :: < Self > ()) } fn IActivationFactory < R , F : FnOnce (& windows_core :: imp :: IGenericFactory) -> windows_core :: Result < R > , > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Reference , windows_core :: imp :: IGenericFactory , > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } pub fn Method < P0 > (& self , stringable : P0) -> windows_core :: Result < windows_core :: HSTRING > where P0 : windows_core :: Param < windows :: Foundation :: IStringable > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Method) (windows_core :: Interface :: as_raw (this) , stringable . param () . abi () , & mut result__ ,) . map (| | core :: mem :: transmute (result__)) } } pub fn ToString (& self) -> windows_core :: Result < windows_core :: HSTRING > { let this = & windows_core :: Interface :: cast :: < windows :: Foundation :: IStringable > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . ToString) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | core :: mem :: transmute (result__)) } } }
};
}
