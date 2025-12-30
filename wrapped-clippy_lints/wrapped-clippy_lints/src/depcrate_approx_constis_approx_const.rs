// Generated macro for is_approx_const (function)
macro_rules! Depcrate_approx_constis_approx_const {
() => {
// Module: crate::approx_const
// Provides: {"is_approx_const"}
// Dependencies: {}
# [doc = " Returns `false` if the number of significant figures in `value` are"] # [doc = " less than `min_digits`; otherwise, returns true if `value` is equal"] # [doc = " to `constant`, rounded to the number of significant digits present in `value`."] # [must_use] fn is_approx_const (constant : f64 , value : & str , f_value : f64 , min_digits : usize) -> bool { if value . len () <= min_digits { false } else if f_value . to_string () . len () > min_digits && constant . to_string () . starts_with (& f_value . to_string ()) { true } else { let round_const = format ! ("{constant:0value_len$.value_prec$}" , value_len = value . len () , value_prec = count_digits_after_dot (value)) ; value == round_const } }
};
}
