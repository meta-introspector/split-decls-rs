// Generated macro for draw_card_back (function)
macro_rules! Depcratedraw_card_back {
() => {
// Module: crate
// Provides: {"draw_card_back"}
// Dependencies: {}
fn draw_card_back (surface : & IDCompositionSurface , bitmap : & ID2D1Bitmap1 , offset : (f32 , f32) , dpi : (f32 , f32) ,) -> Result < () > { unsafe { let mut dc_offset = Default :: default () ; let dc : ID2D1DeviceContext = surface . BeginDraw (None , & mut dc_offset) ? ; dc . SetDpi (dpi . 0 , dpi . 1) ; dc . SetTransform (& Matrix3x2 :: translation (physical_to_logical (dc_offset . x as f32 , dpi . 0) , physical_to_logical (dc_offset . y as f32 , dpi . 1) ,)) ; let left = physical_to_logical (offset . 0 , dpi . 0) ; let top = physical_to_logical (offset . 1 , dpi . 1) ; dc . DrawBitmap (bitmap , None , 1.0 , D2D1_INTERPOLATION_MODE_LINEAR , Some (& D2D_RECT_F { left , top , right : left + CARD_WIDTH , bottom : top + CARD_HEIGHT , }) , None ,) ; surface . EndDraw () } }
};
}
