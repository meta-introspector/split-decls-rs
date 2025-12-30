// Generated macro for impl_311 (impl)
macro_rules! Depcrate_colorimpl_311 {
() => {
// Module: crate::color
// Provides: {"impl_311"}
// Dependencies: {}
impl < T : Primitive > Blend for Rgba < T > { fn blend (& mut self , other : & Rgba < T >) { if other . 0 [3] . is_zero () { return ; } if other . 0 [3] == T :: DEFAULT_MAX_VALUE { * self = * other ; return ; } let max_t = T :: DEFAULT_MAX_VALUE ; let max_t = max_t . to_f32 () . unwrap () ; let (bg_r , bg_g , bg_b , bg_a) = (self . 0 [0] , self . 0 [1] , self . 0 [2] , self . 0 [3]) ; let (fg_r , fg_g , fg_b , fg_a) = (other . 0 [0] , other . 0 [1] , other . 0 [2] , other . 0 [3]) ; let (bg_r , bg_g , bg_b , bg_a) = (bg_r . to_f32 () . unwrap () / max_t , bg_g . to_f32 () . unwrap () / max_t , bg_b . to_f32 () . unwrap () / max_t , bg_a . to_f32 () . unwrap () / max_t ,) ; let (fg_r , fg_g , fg_b , fg_a) = (fg_r . to_f32 () . unwrap () / max_t , fg_g . to_f32 () . unwrap () / max_t , fg_b . to_f32 () . unwrap () / max_t , fg_a . to_f32 () . unwrap () / max_t ,) ; let alpha_final = bg_a + fg_a - bg_a * fg_a ; if alpha_final == 0.0 { return ; } ; let (bg_r_a , bg_g_a , bg_b_a) = (bg_r * bg_a , bg_g * bg_a , bg_b * bg_a) ; let (fg_r_a , fg_g_a , fg_b_a) = (fg_r * fg_a , fg_g * fg_a , fg_b * fg_a) ; let (out_r_a , out_g_a , out_b_a) = (fg_r_a + bg_r_a * (1.0 - fg_a) , fg_g_a + bg_g_a * (1.0 - fg_a) , fg_b_a + bg_b_a * (1.0 - fg_a) ,) ; let (out_r , out_g , out_b) = (out_r_a / alpha_final , out_g_a / alpha_final , out_b_a / alpha_final ,) ; * self = Rgba ([NumCast :: from (max_t * out_r) . unwrap () , NumCast :: from (max_t * out_g) . unwrap () , NumCast :: from (max_t * out_b) . unwrap () , NumCast :: from (max_t * alpha_final) . unwrap () ,]) ; } }
};
}
