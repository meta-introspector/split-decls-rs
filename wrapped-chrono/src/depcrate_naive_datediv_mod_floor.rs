// Generated macro for div_mod_floor (function)
macro_rules! Depcrate_naive_datediv_mod_floor {
() => {
// Module: crate::naive::date
// Provides: {"div_mod_floor"}
// Dependencies: {}
const fn div_mod_floor (val : i32 , div : i32) -> (i32 , i32) { (val . div_euclid (div) , val . rem_euclid (div)) }
};
}
