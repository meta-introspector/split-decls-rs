// Generated macro for impl_244 (impl)
macro_rules! Depcrateimpl_244 {
() => {
// Module: crate
// Provides: {"impl_244"}
// Dependencies: {}
impl < T > ArcSwapOption < T > { # [doc = " A const-fn equivalent of [empty]."] # [doc = ""] # [doc = " Just like [empty], this creates an `None`-holding `ArcSwapOption`. The [empty] is, however,"] # [doc = " more general ‒ this is available only for the default strategy, while [empty] is for any"] # [doc = " [Default]-constructible strategy (current or future one)."] # [doc = ""] # [doc = " [empty]: ArcSwapAny::empty"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::sync::Arc;"] # [doc = " # use arc_swap::ArcSwapOption;"] # [doc = " static GLOBAL_DATA: ArcSwapOption<usize> = ArcSwapOption::const_empty();"] # [doc = ""] # [doc = " assert!(GLOBAL_DATA.load().is_none());"] # [doc = " GLOBAL_DATA.store(Some(Arc::new(42)));"] # [doc = " assert_eq!(42, **GLOBAL_DATA.load().as_ref().unwrap());"] # [doc = " ```"] pub const fn const_empty () -> Self { Self { ptr : AtomicPtr :: new (ptr :: null_mut ()) , _phantom_arc : PhantomData , strategy : HybridStrategy { _config : DefaultConfig , } , } } }
};
}
