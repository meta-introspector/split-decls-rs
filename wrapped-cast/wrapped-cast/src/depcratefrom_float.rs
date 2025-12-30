// Generated macro for from_float (macro)
macro_rules! Depcratefrom_float {
() => {
// Module: crate
// Provides: {"from_float"}
// Dependencies: {}
# [doc = " From a float `$src` to an integer `$dst`"] macro_rules ! from_float { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { $ ($ (impl From <$ src > for $ dst { type Output = Result <$ dst , Error >; # [inline] fn cast (src : $ src) -> Self :: Output { use core :: { $ dst , $ src } ; Err (if src != src { Error :: NaN } else if src == $ src :: INFINITY || src == $ src :: NEG_INFINITY { Error :: Infinite } else if { let dst_bits = core :: mem :: size_of ::<$ dst > () as u32 * 8 ; let lossless = dst_bits < core ::$ src :: MANTISSA_DIGITS ; let max = if lossless { $ dst :: MAX as $ src } else { $ src :: from_bits (($ dst :: MAX as $ src) . to_bits () - 1) } ; src > max } { Error :: Overflow } else if $ dst :: MIN == 0 { if src <= - 1.0 { Error :: Underflow } else { return Ok (src as $ dst) ; } } else if src < $ dst :: MIN as $ src { Error :: Underflow } else { return Ok (src as $ dst) ; }) } }) +) + } }
};
}
