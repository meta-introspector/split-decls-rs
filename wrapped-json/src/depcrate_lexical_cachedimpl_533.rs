// Generated macro for impl_533 (impl)
macro_rules! Depcrate_lexical_cachedimpl_533 {
() => {
// Module: crate::lexical::cached
// Provides: {"impl_533"}
// Dependencies: {}
# [doc = " Allow indexing of values without bounds checking"] impl ExtendedFloatArray { # [inline] pub fn get_extended_float (& self , index : usize) -> ExtendedFloat { let mant = self . mant [index] ; let exp = self . exp [index] ; ExtendedFloat { mant , exp } } # [inline] pub fn len (& self) -> usize { self . mant . len () } }
};
}
