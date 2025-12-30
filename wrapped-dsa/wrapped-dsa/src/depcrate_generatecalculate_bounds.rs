// Generated macro for calculate_bounds (function)
macro_rules! Depcrate_generatecalculate_bounds {
() => {
// Module: crate::generate
// Provides: {"calculate_bounds"}
// Dependencies: {}
# [doc = " Calculate the upper and lower bounds for generating values like p or q"] # [inline] fn calculate_bounds (size : u32) -> (NonZero < BoxedUint > , NonZero < BoxedUint >) { let lower = BoxedUint :: one () . resize (size + 1) . shl (size - 1) ; let upper = BoxedUint :: one () . resize (size + 1) . shl (size) ; let lower = NonZero :: new (lower) . expect ("[bug] shl can't go backward") ; let upper = NonZero :: new (upper) . expect ("[bug] shl can't go backward") ; (lower , upper) }
};
}
