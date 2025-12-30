// Generated macro for theta_to_ordinal_coord (function)
macro_rules! Depcrate_element_pietheta_to_ordinal_coord {
() => {
// Module: crate::element::pie
// Provides: {"theta_to_ordinal_coord"}
// Dependencies: {}
fn theta_to_ordinal_coord (radius : f64 , theta : f64 , ordinal_offset : & (i32 , i32)) -> (i32 , i32) { let (sin , cos) = theta . sin_cos () ; ((radius * cos + ordinal_offset . 0 as f64) . round () as i32 , (radius * sin + ordinal_offset . 1 as f64) . round () as i32 ,) }
};
}
