// Generated macro for boxes_for_gauss (function)
macro_rules! Depcrate_imageops_fast_blurboxes_for_gauss {
() => {
// Module: crate::imageops::fast_blur
// Provides: {"boxes_for_gauss"}
// Dependencies: {}
fn boxes_for_gauss (sigma : f32 , n : usize) -> Vec < usize > { let w_ideal = f32 :: sqrt ((12.0 * sigma . powi (2) / (n as f32)) + 1.0) ; let mut w_l = w_ideal . floor () ; if w_l % 2.0 == 0.0 { w_l -= 1.0 ; } ; let w_u = w_l + 2.0 ; let m_ideal = 0.25 * (n as f32) * (w_l + 3.0) - 3.0 * sigma . powi (2) * (w_l + 1.0) . recip () ; let m = f32 :: round (m_ideal) as usize ; (0 .. n) . map (| i | if i < m { w_l as usize } else { w_u as usize }) . collect :: < Vec < _ > > () }
};
}
