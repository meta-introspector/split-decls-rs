// Generated macro for float_to_weight (function)
macro_rules! Depcrate_strategy_unionsfloat_to_weight {
() => {
// Module: crate::strategy::unions
// Provides: {"float_to_weight"}
// Dependencies: {}
# [doc = " Convert a floating-point weight in the range (0.0,1.0) to a pair of weights"] # [doc = " that can be used with `Union` and similar."] # [doc = ""] # [doc = " The first return value is the weight corresponding to `f`; the second"] # [doc = " return value is the weight corresponding to `1.0 - f`."] # [doc = ""] # [doc = " This call does not make any guarantees as to what range of weights it may"] # [doc = " produce, except that adding the two return values will never overflow a"] # [doc = " `u32`. As such, it is generally not meaningful to combine any other weights"] # [doc = " with the two returned."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " Panics if `f` is not a real number between 0.0 and 1.0, both exclusive."] pub fn float_to_weight (f : f64) -> (u32 , u32) { assert ! (f > 0.0 && f < 1.0 , "Invalid probability: {}" , f) ; let pos = max (1 , min (WEIGHT_BASE - 1 , (f * f64 :: from (WEIGHT_BASE)) . round () as u32) ,) ; let neg = WEIGHT_BASE - pos ; (pos , neg) }
};
}
