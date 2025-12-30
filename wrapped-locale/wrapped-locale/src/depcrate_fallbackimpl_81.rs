// Generated macro for impl_81 (impl)
macro_rules! Depcrate_fallbackimpl_81 {
() => {
// Module: crate::fallback
// Provides: {"impl_81"}
// Dependencies: {}
impl LocaleFallbackIterator < '_ > { # [doc = " Borrows the current [`DataLocale`] under fallback."] pub fn get (& self) -> & DataLocale { & self . current } # [doc = " Takes the current [`DataLocale`] under fallback."] pub fn take (self) -> DataLocale { self . current } # [doc = " Performs one step of the locale fallback algorithm."] # [doc = ""] # [doc = " The fallback is completed once the inner [`DataLocale`] becomes [`DataLocale::default()`]."] pub fn step (& mut self) -> & mut Self { self . inner . step (& mut self . current) ; self } }
};
}
