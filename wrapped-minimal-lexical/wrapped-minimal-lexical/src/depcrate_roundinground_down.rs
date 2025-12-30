// Generated macro for round_down (function)
macro_rules! Depcrate_roundinground_down {
() => {
// Module: crate::rounding
// Provides: {"round_down"}
// Dependencies: {}
# [doc = " Round our significant digits into place, truncating them."] # [cfg_attr (not (feature = "compact") , inline)] pub fn round_down (fp : & mut ExtendedFloat , shift : i32) { fp . mant = match shift == 64 { true => 0 , false => fp . mant >> shift , } ; fp . exp += shift ; }
};
}
