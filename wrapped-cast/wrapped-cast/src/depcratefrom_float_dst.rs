// Generated macro for from_float_dst (macro)
macro_rules! Depcratefrom_float_dst {
() => {
// Module: crate
// Provides: {"from_float_dst"}
// Dependencies: {}
# [doc = " From a float `$src` to an integer `$dst`, where $dst is large enough to contain"] # [doc = " all values of `$src`. We can't ever overflow here"] macro_rules ! from_float_dst { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { $ ($ (impl From <$ src > for $ dst { type Output = Result <$ dst , Error >; # [inline] # [allow (unused_comparisons)] fn cast (src : $ src) -> Self :: Output { use core :: { $ dst , $ src } ; Err (if src != src { Error :: NaN } else if src == $ src :: INFINITY || src == $ src :: NEG_INFINITY { Error :: Infinite } else if ($ dst :: MIN == 0) && src <= - 1.0 { Error :: Underflow } else { return Ok (src as $ dst) ; }) } }) +) + } }
};
}
