// Generated macro for decide (function)
macro_rules! Depcratedecide {
() => {
// Module: crate
// Provides: {"decide"}
// Dependencies: {}
fn decide (recent_cost : u32 , max_cost : u32 , mut rand_float : impl FnMut () -> f32) -> bool { let load = if max_cost == 0 || recent_cost >= max_cost { return false ; } else { f64 :: from (recent_cost) / f64 :: from (max_cost) } ; let linear_reject_prob = (load - 0.75) * 4.0 ; if linear_reject_prob <= 0.0 { return true ; } let reject_prob = linear_reject_prob . powi (2) ; reject_prob < rand_float () . into () }
};
}
