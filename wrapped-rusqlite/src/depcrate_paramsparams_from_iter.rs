// Generated macro for params_from_iter (function)
macro_rules! Depcrate_paramsparams_from_iter {
() => {
// Module: crate::params
// Provides: {"params_from_iter"}
// Dependencies: {}
# [doc = " Constructor function for a [`ParamsFromIter`]. See its documentation for"] # [doc = " more."] # [inline] pub fn params_from_iter < I > (iter : I) -> ParamsFromIter < I > where I : IntoIterator , I :: Item : ToSql , { ParamsFromIter (iter) }
};
}
