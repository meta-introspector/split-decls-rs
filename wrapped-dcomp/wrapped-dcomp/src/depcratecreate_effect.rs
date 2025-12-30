// Generated macro for create_effect (function)
macro_rules! Depcratecreate_effect {
() => {
// Module: crate
// Provides: {"create_effect"}
// Dependencies: {}
fn create_effect (device : & IDCompositionDesktopDevice , visual : & IDCompositionVisual2 , rotation : & IDCompositionRotateTransform3D , front : bool , dpi : (f32 , f32) ,) -> Result < () > { unsafe { let width = logical_to_physical (CARD_WIDTH , dpi . 0) ; let height = logical_to_physical (CARD_HEIGHT , dpi . 1) ; let pre_matrix = Matrix4x4 :: translation (- width / 2.0 , - height / 2.0 , 0.0) * Matrix4x4 :: rotation_y (if front { 180.0 } else { 0.0 }) ; let pre_transform = device . CreateMatrixTransform3D () ? ; pre_transform . SetMatrix (& pre_matrix) ? ; let post_matrix = Matrix4x4 :: perspective_projection (width * 2.0) * Matrix4x4 :: translation (width / 2.0 , height / 2.0 , 0.0) ; let post_transform = device . CreateMatrixTransform3D () ? ; post_transform . SetMatrix (& post_matrix) ? ; let transform = device . CreateTransform3DGroup (& [pre_transform . cast () . ok () , rotation . cast () . ok () , post_transform . cast () . ok () ,]) ? ; visual . SetEffect (& transform) } }
};
}
