// Generated macro for fraction_of_products_of_factorials (function)
macro_rules! Depcrate_hypergeometricfraction_of_products_of_factorials {
() => {
// Module: crate::hypergeometric
// Provides: {"fraction_of_products_of_factorials"}
// Dependencies: {}
fn fraction_of_products_of_factorials (numerator : (u64 , u64) , denominator : (u64 , u64)) -> f64 { let min_top = u64 :: min (numerator . 0 , numerator . 1) ; let min_bottom = u64 :: min (denominator . 0 , denominator . 1) ; let min_all = u64 :: min (min_top , min_bottom) ; let max_top = u64 :: max (numerator . 0 , numerator . 1) ; let max_bottom = u64 :: max (denominator . 0 , denominator . 1) ; let max_all = u64 :: max (max_top , max_bottom) ; let mut result = 1.0 ; for i in (min_all + 1) ..= max_all { if i <= min_top { result *= i as f64 ; } if i <= min_bottom { result /= i as f64 ; } if i <= max_top { result *= i as f64 ; } if i <= max_bottom { result /= i as f64 ; } } result }
};
}
