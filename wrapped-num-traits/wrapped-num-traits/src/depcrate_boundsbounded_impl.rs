// Generated macro for bounded_impl (macro)
macro_rules! Depcrate_boundsbounded_impl {
() => {
// Module: crate::bounds
// Provides: {"bounded_impl"}
// Dependencies: {}
macro_rules ! bounded_impl { ($ t : ty , $ min : expr , $ max : expr) => { impl Bounded for $ t { # [inline] fn min_value () -> $ t { $ min } # [inline] fn max_value () -> $ t { $ max } } } ; }
};
}
