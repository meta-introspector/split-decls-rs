// Generated macro for render (function)
macro_rules! Depcrate_renderrender {
() => {
// Module: crate::render
// Provides: {"render"}
// Dependencies: {}
pub fn render (scene : & dyn Model , camera : & Camera , width : usize , height : usize , samples : usize ,) -> Vec < RGB < u8 > > { let mut pixels : Vec < RGB < u8 > > = Vec :: with_capacity (width * height) ; for y in 0 .. height { let j = height - 1 - y ; for i in 0 .. width { let mut col = Vec3 (0.0 , 0.0 , 0.0) ; for _ in 0 .. samples { let u = (i as f32 + 0.5) / width as f32 ; let v = (j as f32 + 0.5) / height as f32 ; let r = camera . get_ray (u , v) ; col = col + color (r , scene) ; } col = col / samples as f32 ; col = Vec3 (col . x () . sqrt () , col . y () . sqrt () , col . z () . sqrt ()) ; let rgb = col . to_u8 () ; pixels . push (RGB { r : rgb [0] , g : rgb [1] , b : rgb [2] , }) ; } } pixels }
};
}
