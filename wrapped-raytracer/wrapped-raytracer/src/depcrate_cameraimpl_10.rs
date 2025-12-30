// Generated macro for impl_10 (impl)
macro_rules! Depcrate_cameraimpl_10 {
() => {
// Module: crate::camera
// Provides: {"impl_10"}
// Dependencies: {}
impl Camera { pub fn new (lookfrom : Vec3 , lookat : Vec3 , vup : Vec3 , vfov_degrees : f32 , aspect : f32 , aperture : f32 , focus_distance : f32 ,) -> Camera { let theta = vfov_degrees * PI / 180.0 ; let half_height = (theta / 2.0) . tan () ; let half_width = aspect * half_height ; let w = (lookfrom - lookat) . to_unit_vector () ; let u = vup . cross (w) . to_unit_vector () ; let v = w . cross (u) ; Camera { origin : lookfrom , u , v , lower_left_corner : lookfrom - focus_distance * (half_width * u + half_height * v + w) , horizontal : focus_distance * 2.0 * half_width * u , vertical : focus_distance * 2.0 * half_height * v , lens_radius : aperture / 2.0 , } } pub fn get_ray (& self , u : f32 , v : f32) -> Ray { let Vec3 (du , dv , _) = self . lens_radius * random_in_unit_disc () ; let origin = self . origin + du * self . u + dv * self . v ; Ray :: new (origin , self . lower_left_corner + u * self . horizontal + v * self . vertical - origin ,) } }
};
}
