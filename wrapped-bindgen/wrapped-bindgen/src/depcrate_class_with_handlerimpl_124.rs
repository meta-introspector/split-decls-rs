// Generated macro for impl_124 (impl)
macro_rules! Depcrate_class_with_handlerimpl_124 {
() => {
// Module: crate::class_with_handler
// Provides: {"impl_124"}
// Dependencies: {}
impl Deferral { pub fn Close (& self) -> windows_core :: Result < () > { let this = & windows_core :: Interface :: cast :: < IClosable > (self) ? ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } pub fn Complete (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Complete) (windows_core :: Interface :: as_raw (this)) . ok () } } pub fn Create < P0 > (handler : P0) -> windows_core :: Result < Deferral > where P0 : windows_core :: Param < DeferralCompletedHandler > , { Self :: IDeferralFactory (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Create) (windows_core :: Interface :: as_raw (this) , handler . param () . abi () , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) }) } fn IDeferralFactory < R , F : FnOnce (& IDeferralFactory) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Deferral , IDeferralFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
