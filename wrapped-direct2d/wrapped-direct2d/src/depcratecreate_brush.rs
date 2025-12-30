// Generated macro for create_brush (function)
macro_rules! Depcratecreate_brush {
() => {
// Module: crate
// Provides: {"create_brush"}
// Dependencies: {}
fn create_brush (target : & ID2D1DeviceContext) -> Result < ID2D1SolidColorBrush > { let color = D2D1_COLOR_F { r : 0.92 , g : 0.38 , b : 0.208 , a : 1.0 , } ; let properties = D2D1_BRUSH_PROPERTIES { opacity : 0.8 , transform : Matrix3x2 :: identity () , } ; unsafe { target . CreateSolidColorBrush (& color , Some (& properties)) } }
};
}
