// Generated macro for ProjectionMatrixBuilder (struct)
macro_rules! Depcrate_coord_ranged3d_projectionProjectionMatrixBuilder {
() => {
// Module: crate::coord::ranged3d::projection
// Provides: {"ProjectionMatrixBuilder"}
// Dependencies: {}
# [doc = " The helper struct to build a projection matrix"] # [derive (Copy , Clone)] pub struct ProjectionMatrixBuilder { # [doc = " Specifies the yaw of the 3D coordinate system"] pub yaw : f64 , # [doc = " Specifies the pitch of the 3D coordinate system"] pub pitch : f64 , # [doc = " Specifies the scale of the 3D coordinate system"] pub scale : f64 , pivot_before : (i32 , i32 , i32) , pivot_after : (i32 , i32) , }
};
}
