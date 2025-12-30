// Generated macro for Integer (struct)
macro_rules! Depcrate_integerInteger {
() => {
// Module: crate::integer
// Provides: {"Integer"}
// Dependencies: {}
# [doc = " An integer literal."] # [derive (Debug)] pub (crate) struct Integer { # [doc = " Whether the integer is negative."] pub (crate) is_negative : bool , # [doc = " The value after casting to `u128`."] pub (crate) raw_value : u128 , # [doc = " The suffix, whether `u`, `i`, or omitted."] pub (crate) suffix : Suffix , # [doc = " The span of the integer literal."] pub (crate) span : Span , }
};
}
