// Generated macro for calculate_bound_u32 (function)
macro_rules! Depcrate_seq_increasing_uniformcalculate_bound_u32 {
() => {
// Module: crate::seq::increasing_uniform
// Provides: {"calculate_bound_u32"}
// Dependencies: {}
# [inline] # [doc = " Calculates `bound`, `count` such that bound (m)*(m+1)*..*(m + remaining - 1)"] fn calculate_bound_u32 (m : u32) -> (u32 , u8) { debug_assert ! (m > 0) ; # [inline] const fn inner (m : u32) -> (u32 , u8) { let mut product = m ; let mut current = m + 1 ; loop { if let Some (p) = u32 :: checked_mul (product , current) { product = p ; current += 1 ; } else { let count = (current - m) as u8 ; return (product , count) ; } } } const RESULT2 : (u32 , u8) = inner (2) ; if m == 2 { return RESULT2 ; } inner (m) }
};
}
