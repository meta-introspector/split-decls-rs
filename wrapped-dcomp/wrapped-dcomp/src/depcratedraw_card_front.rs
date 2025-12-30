// Generated macro for draw_card_front (function)
macro_rules! Depcratedraw_card_front {
() => {
// Module: crate
// Provides: {"draw_card_front"}
// Dependencies: {}
fn draw_card_front (surface : & IDCompositionSurface , value : u8 , format : & IDWriteTextFormat , brush : & ID2D1SolidColorBrush , dpi : (f32 , f32) ,) -> Result < () > { unsafe { let mut offset = Default :: default () ; let dc : ID2D1DeviceContext = surface . BeginDraw (None , & mut offset) ? ; dc . SetDpi (dpi . 0 , dpi . 1) ; dc . SetTransform (& Matrix3x2 :: translation (physical_to_logical (offset . x as f32 , dpi . 0) , physical_to_logical (offset . y as f32 , dpi . 1) ,)) ; dc . Clear (Some (& D2D1_COLOR_F { r : 1.0 , g : 1.0 , b : 1.0 , a : 1.0 , })) ; dc . DrawText (& [value as _] , format , & D2D_RECT_F { left : 0.0 , top : 0.0 , right : CARD_WIDTH , bottom : CARD_HEIGHT , } , brush , D2D1_DRAW_TEXT_OPTIONS_NONE , DWRITE_MEASURING_MODE_NATURAL ,) ; surface . EndDraw () } }
};
}
