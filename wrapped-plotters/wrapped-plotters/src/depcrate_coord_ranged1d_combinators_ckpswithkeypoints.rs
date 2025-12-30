// Generated macro for WithKeyPoints (struct)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsWithKeyPoints {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"WithKeyPoints"}
// Dependencies: {}
# [doc = " The coordinate decorator that binds a key point vector."] # [doc = " Normally, all the ranged coordinate implements its own keypoint algorithm"] # [doc = " to determine how to render the tick mark and mesh grid."] # [doc = " This decorator allows customized tick mark specifiied by vector."] # [doc = " See [BindKeyPoints::with_key_points](trait.BindKeyPoints.html#tymethod.with_key_points)"] # [doc = " for details."] # [doc = " Note: For any coordinate spec wrapped by this decorator, the maximum number of labels configured by"] # [doc = " MeshStyle will be ignored and the key point function will always returns the entire vector"] # [derive (Clone)] pub struct WithKeyPoints < Inner : Ranged > { inner : Inner , bold_points : Vec < Inner :: ValueType > , light_points : Vec < Inner :: ValueType > , }
};
}
