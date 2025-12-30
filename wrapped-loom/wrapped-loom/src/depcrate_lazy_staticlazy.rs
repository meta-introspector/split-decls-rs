// Generated macro for Lazy (struct)
macro_rules! Depcrate_lazy_staticLazy {
() => {
// Module: crate::lazy_static
// Provides: {"Lazy"}
// Dependencies: {}
# [doc = " Mock implementation of `lazy_static::Lazy`."] pub struct Lazy < T > { # [doc (hidden)] pub init : fn () -> T , # [doc (hidden)] pub _p : PhantomData < fn (T) > , }
};
}
