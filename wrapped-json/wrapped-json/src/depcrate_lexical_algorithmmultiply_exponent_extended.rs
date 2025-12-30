// Generated macro for multiply_exponent_extended (function)
macro_rules! Depcrate_lexical_algorithmmultiply_exponent_extended {
() => {
// Module: crate::lexical::algorithm
// Provides: {"multiply_exponent_extended"}
// Dependencies: {}
# [doc = " Multiply the floating-point by the exponent."] # [doc = ""] # [doc = " Multiply by pre-calculated powers of the base, modify the extended-"] # [doc = " float, and return if new value and if the value can be represented"] # [doc = " accurately."] fn multiply_exponent_extended < F > (fp : & mut ExtendedFloat , exponent : i32 , truncated : bool) -> bool where F : Float , { let powers = ExtendedFloat :: get_powers () ; let exponent = exponent . saturating_add (powers . bias) ; let small_index = exponent % powers . step ; let large_index = exponent / powers . step ; if exponent < 0 { fp . mant = 0 ; true } else if large_index as usize >= powers . large . len () { fp . mant = 1 << 63 ; fp . exp = 0x7FF ; true } else { let mut errors : u32 = 0 ; if truncated { errors += u64 :: error_halfscale () ; } match fp . mant . overflowing_mul (powers . get_small_int (small_index as usize)) { (_ , true) => { fp . normalize () ; fp . imul (& powers . get_small (small_index as usize)) ; errors += u64 :: error_halfscale () ; } (mant , false) => { fp . mant = mant ; fp . normalize () ; } } fp . imul (& powers . get_large (large_index as usize)) ; if errors > 0 { errors += 1 ; } errors += u64 :: error_halfscale () ; let shift = fp . normalize () ; errors <<= shift ; u64 :: error_is_accurate :: < F > (errors , fp) } }
};
}
