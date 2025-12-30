// Generated macro for color (function)
macro_rules! Depcrate_rendercolor {
() => {
// Module: crate::render
// Provides: {"color"}
// Dependencies: {}
fn color (mut r : Ray , model : & dyn Model) -> Vec3 { const WHITE : Vec3 = Vec3 (1.0 , 1.0 , 1.0) ; let sky_blue = 0.3 * Vec3 (0.5 , 0.7 , 1.0) + 0.7 * WHITE ; let mut attenuation = WHITE ; let mut depth = 0 ; while let Some (hit) = model . hit (& r) { let scattered = hit . material . scatter (& r , & hit) ; attenuation = attenuation * scattered . color ; if let Some (bounce) = scattered . ray { r = bounce ; } else { break ; } depth += 1 ; if depth >= 50 { break ; } } let sun_direction = Vec3 (1.0 , 1.0 , 1.0) . to_unit_vector () ; let unit_direction = r . direction . to_unit_vector () ; if sun_direction . dot (unit_direction) >= (5.0 * PI / 180.0) . cos () { Vec3 (5.0 , 5.0 , 3.0) * attenuation } else { let t = 0.5 * (unit_direction . y () + 1.0) ; let orig_color = (1.0 - t) * WHITE + t * sky_blue ; orig_color * attenuation } }
};
}
