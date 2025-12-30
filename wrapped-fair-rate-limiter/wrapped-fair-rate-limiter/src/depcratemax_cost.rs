// Generated macro for max_cost (function)
macro_rules! Depcratemax_cost {
() => {
// Module: crate
// Provides: {"max_cost"}
// Dependencies: {}
# [doc = " When recent load is in (0.75,1.0], linearly interpolate max cost between"] # [doc = " `global_max_cost` and `global_max_cost`/keys."] # [allow (clippy :: cast_possible_truncation , clippy :: cast_sign_loss)] fn max_cost (sources_max : u32 , recent_cost : u32 , keys : u32) -> u32 { if sources_max < 1 { return 0 ; } let load = f64 :: from (recent_cost) / f64 :: from (sources_max) ; if keys < 1 { sources_max } else if load > 1.0 { (f64 :: from (sources_max) / f64 :: from (keys)) as u32 } else if load > 0.75 { let x = (load - 0.75) * 4.0 ; (f64 :: from (sources_max) * (1.0 - (1.0 - 1.0 / f64 :: from (keys)) * x)) as u32 } else { sources_max } }
};
}
