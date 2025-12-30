// Generated macro for WithKeyPointMethod (struct)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsWithKeyPointMethod {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"WithKeyPointMethod"}
// Dependencies: {}
# [doc = " The coordinate decorator that allows customized keypoint algorithms."] # [doc = " Normally, all the coordinate spec implements its own key point algorithm"] # [doc = " But this decorator allows you override the pre-defined key point algorithm."] # [doc = ""] # [doc = " To use this decorator, see [BindKeyPointMethod::with_key_point_func](trait.BindKeyPointMethod.html#tymethod.with_key_point_func)"] pub struct WithKeyPointMethod < R : Ranged > { inner : R , bold_func : Box < dyn Fn (usize) -> Vec < R :: ValueType > > , light_func : Box < dyn Fn (usize) -> Vec < R :: ValueType > > , }
};
}
