// Generated macro for Internable (trait)
macro_rules! Depcrate_utils_cacheInternable {
() => {
// Module: crate::utils::cache
// Provides: {"Internable"}
// Dependencies: {}
# [doc = " Defines the behavior required for a type to be internable."] # [doc = ""] # [doc = " Types implementing this trait must provide access to a static cache and define an `intern` method"] # [doc = " that ensures values are stored uniquely."] trait Internable : Clone + Eq + Hash + 'static { fn intern_cache () -> & 'static Mutex < TyIntern < Self > > ; fn intern (self) -> Interned < Self > { Self :: intern_cache () . lock () . unwrap () . intern (self) } }
};
}
