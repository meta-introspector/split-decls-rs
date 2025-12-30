// Generated macro for impl_821 (impl)
macro_rules! Depcrate_style_colorimpl_821 {
() => {
// Module: crate::style::color
// Provides: {"impl_821"}
// Dependencies: {}
impl Color for HSLColor { # [inline (always)] # [allow (clippy :: many_single_char_names)] fn to_backend_color (& self) -> BackendColor { let (h , s , l) = (self . 0 . clamp (0.0 , 1.0) , self . 1 . clamp (0.0 , 1.0) , self . 2 . clamp (0.0 , 1.0) ,) ; if s == 0.0 { let value = (l * 255.0) . round () as u8 ; return BackendColor { rgb : (value , value , value) , alpha : 1.0 , } ; } let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s } ; let p = 2.0 * l - q ; let cvt = | mut t | { if t < 0.0 { t += 1.0 ; } if t > 1.0 { t -= 1.0 ; } let value = if t < 1.0 / 6.0 { p + (q - p) * 6.0 * t } else if t < 1.0 / 2.0 { q } else if t < 2.0 / 3.0 { p + (q - p) * (2.0 / 3.0 - t) * 6.0 } else { p } ; (value * 255.0) . round () as u8 } ; BackendColor { rgb : (cvt (h + 1.0 / 3.0) , cvt (h) , cvt (h - 1.0 / 3.0)) , alpha : 1.0 , } } }
};
}
