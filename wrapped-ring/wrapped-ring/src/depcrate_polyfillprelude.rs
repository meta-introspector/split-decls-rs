// Generated macro for prelude (module)
macro_rules! Depcrate_polyfillprelude {
() => {
// Module: crate::polyfill
// Provides: {"prelude"}
// Dependencies: {}
# [allow (unused_imports)] pub mod prelude { pub (crate) use super :: { atomic :: AtomicPolyfills , boxed :: { BoxMaybeUninitSlicePolyfills , BoxSlicePolyfills } , ptr :: { ConstPointerPolyfills , PointerPolyfills } , slice :: { SliceOfArraysPolyfills , SlicePolyfills } , } ; }
};
}
