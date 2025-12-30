// Generated macro for T_MIN (const)
macro_rules! Depcrate_modelT_MIN {
() => {
// Module: crate::model
// Provides: {"T_MIN"}
// Dependencies: {}
# [doc = " Minimum distance a ray must travel before we'll consider a possible hit."] # [doc = ""] # [doc = " If we try to use 0 here, we get a really strange bug. When a ray hits an object"] # [doc = " and bounces, we'll sometimes register another hit on the same sphere,"] # [doc = " at some tiny but positive distance, due to floating-point error."] # [doc = ""] const T_MIN : f32 = 0.0001 ;
};
}
