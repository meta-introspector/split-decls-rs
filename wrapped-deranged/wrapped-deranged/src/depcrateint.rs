// Generated macro for int (macro)
macro_rules! Depcrateint {
() => {
// Module: crate
// Provides: {"int"}
// Dependencies: {}
# [doc = " A macro to define a ranged integer with an automatically computed inner type."] # [doc = ""] # [doc = " The minimum and maximum values are provided as integer literals, and the macro will compute an"] # [doc = " appropriate inner type to represent the range. This will be the smallest integer type that can"] # [doc = " store both the minimum and maximum values, with a preference for unsigned types if both are"] # [doc = " possible. To specifically request a signed or unsigned type, you can append a `i` or `u` suffix"] # [doc = " to either or both of the minimum and maximum values, respectively."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " int!(0, 100);  // RangedU8<0, 100>"] # [doc = " int!(0i, 100); // RangedI8<0, 100>"] # [doc = " int!(-5, 5);   // RangedI8<-5, 5>"] # [doc = " int!(-5u, 5);  // compile error (-5 cannot be unsigned)"] # [doc = " ```"] # [cfg (docsrs)] # [doc (cfg (feature = "macros"))] # [macro_export] macro_rules ! int { ($ min : literal , $ max : literal) => { } ; }
};
}
