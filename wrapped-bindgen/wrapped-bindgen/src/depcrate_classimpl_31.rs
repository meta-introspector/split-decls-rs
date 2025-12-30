// Generated macro for impl_31 (impl)
macro_rules! Depcrate_classimpl_31 {
() => {
// Module: crate::class
// Provides: {"impl_31"}
// Dependencies: {}
impl Deferral { pub fn Close (& self) -> windows_core :: Result < () > { let this = & windows_core :: Interface :: cast :: < IClosable > (self) ? ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } pub fn Complete (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Complete) (windows_core :: Interface :: as_raw (this)) . ok () } } fn IDeferralFactory < R , F : FnOnce (& IDeferralFactory) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Deferral , IDeferralFactory > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
