// Generated macro for opt_int (macro)
macro_rules! Depcrateopt_int {
() => {
// Module: crate
// Provides: {"opt_int"}
// Dependencies: {}
# [doc = " A macro to define an optional ranged integer with an automatically computed inner type."] # [doc = ""] # [doc = " The minimum and maximum values are provided as integer literals, and the macro will compute an"] # [doc = " appropriate inner type to represent the range. This will be the smallest integer type that can"] # [doc = " store both the minimum and maximum values, with a preference for unsigned types if both are"] # [doc = " possible. To specifically request a signed or unsigned type, you can append a `i` or `u` suffix"] # [doc = " to either or both of the minimum and maximum values, respectively."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " opt_int!(0, 100);  // OptionRangedU8<0, 100>"] # [doc = " opt_int!(0i, 100); // OptionRangedI8<0, 100>"] # [doc = " opt_int!(-5, 5);   // OptionRangedI8<-5, 5>"] # [doc = " opt_int!(-5u, 5);  // compile error (-5 cannot be unsigned)"] # [doc = " ```"] # [cfg (docsrs)] # [doc (cfg (feature = "macros"))] # [macro_export] macro_rules ! opt_int { ($ min : literal , $ max : literal) => { } ; }
};
}
