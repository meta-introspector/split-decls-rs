// Generated macro for impl_28 (impl)
macro_rules! Depcrate_bindingsimpl_28 {
() => {
// Module: crate::bindings
// Provides: {"impl_28"}
// Dependencies: {}
impl Static { pub fn Property () -> windows_core :: Result < i32 > { Self :: IStaticStatics (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Property) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) }) } fn IStaticStatics < R , F : FnOnce (& IStaticStatics) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < Static , IStaticStatics > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
