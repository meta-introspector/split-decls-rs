// Generated macro for refract (function)
macro_rules! Depcrate_materialsrefract {
() => {
// Module: crate::materials
// Provides: {"refract"}
// Dependencies: {}
fn refract (v : Vec3 , n : Vec3 , ni_over_nt : f32) -> Option < Vec3 > { let uv = v . to_unit_vector () ; let dt = uv . dot (n) ; let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt) ; if discriminant > 0.0 { Some (ni_over_nt * (uv - dt * n) - discriminant . sqrt () * n) } else { None } }
};
}
