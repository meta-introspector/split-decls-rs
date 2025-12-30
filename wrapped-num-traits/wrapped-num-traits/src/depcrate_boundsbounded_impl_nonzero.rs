// Generated macro for bounded_impl_nonzero (macro)
macro_rules! Depcrate_boundsbounded_impl_nonzero {
() => {
// Module: crate::bounds
// Provides: {"bounded_impl_nonzero"}
// Dependencies: {}
macro_rules ! bounded_impl_nonzero { ($ t : ty , $ min : expr , $ max : expr) => { impl Bounded for $ t { # [inline] fn min_value () -> $ t { bounded_impl_nonzero_const ! ($ t , $ min , MIN) ; MIN } # [inline] fn max_value () -> $ t { bounded_impl_nonzero_const ! ($ t , $ max , MAX) ; MAX } } } ; }
};
}
