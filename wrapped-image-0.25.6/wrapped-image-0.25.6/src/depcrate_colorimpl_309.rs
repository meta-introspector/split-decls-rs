// Generated macro for impl_309 (impl)
macro_rules! Depcrate_colorimpl_309 {
() => {
// Module: crate::color
// Provides: {"impl_309"}
// Dependencies: {}
impl < T : Primitive > Blend for LumaA < T > { fn blend (& mut self , other : & LumaA < T >) { let max_t = T :: DEFAULT_MAX_VALUE ; let max_t = max_t . to_f32 () . unwrap () ; let (bg_luma , bg_a) = (self . 0 [0] , self . 0 [1]) ; let (fg_luma , fg_a) = (other . 0 [0] , other . 0 [1]) ; let (bg_luma , bg_a) = (bg_luma . to_f32 () . unwrap () / max_t , bg_a . to_f32 () . unwrap () / max_t ,) ; let (fg_luma , fg_a) = (fg_luma . to_f32 () . unwrap () / max_t , fg_a . to_f32 () . unwrap () / max_t ,) ; let alpha_final = bg_a + fg_a - bg_a * fg_a ; if alpha_final == 0.0 { return ; } ; let bg_luma_a = bg_luma * bg_a ; let fg_luma_a = fg_luma * fg_a ; let out_luma_a = fg_luma_a + bg_luma_a * (1.0 - fg_a) ; let out_luma = out_luma_a / alpha_final ; * self = LumaA ([NumCast :: from (max_t * out_luma) . unwrap () , NumCast :: from (max_t * alpha_final) . unwrap () ,]) ; } }
};
}
