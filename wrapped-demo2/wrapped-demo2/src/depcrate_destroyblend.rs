// Generated macro for blend (function)
macro_rules! Depcrate_destroyblend {
() => {
// Module: crate::destroy
// Provides: {"blend"}
// Dependencies: {}
fn blend (mask_color : Color , cell_color : Color , percentage : f64) -> Color { let Color :: Rgb (mask_red , mask_green , mask_blue) = mask_color else { return mask_color ; } ; let Color :: Rgb (cell_red , cell_green , cell_blue) = cell_color else { return mask_color ; } ; let remain = 1.0 - percentage ; let red = f64 :: from (mask_red) . mul_add (percentage , f64 :: from (cell_red) * remain) ; let green = f64 :: from (mask_green) . mul_add (percentage , f64 :: from (cell_green) * remain) ; let blue = f64 :: from (mask_blue) . mul_add (percentage , f64 :: from (cell_blue) * remain) ; # [expect (clippy :: cast_possible_truncation , clippy :: cast_sign_loss)] Color :: Rgb (red as u8 , green as u8 , blue as u8) }
};
}
