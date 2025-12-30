// Generated macro for parse_big_endian_in_range_and_pad_consttime (function)
macro_rules! Depcrate_limbparse_big_endian_in_range_and_pad_consttime {
() => {
// Module: crate::limb
// Provides: {"parse_big_endian_in_range_and_pad_consttime"}
// Dependencies: {}
# [doc = " Parses `input` into `result`, verifies that the value is less than"] # [doc = " `max_exclusive`, and pads `result` with zeros to its length. If `allow_zero`"] # [doc = " is not `AllowZero::Yes`, zero values are rejected."] # [doc = ""] # [doc = " This attempts to be constant-time with respect to the actual value *only if*"] # [doc = " the value is actually in range. In other words, this won't leak anything"] # [doc = " about a valid value, but it might leak small amounts of information about an"] # [doc = " invalid value (which constraint it failed)."] pub fn parse_big_endian_in_range_and_pad_consttime (input : untrusted :: Input , allow_zero : AllowZero , max_exclusive : & [Limb] , result : & mut [Limb] ,) -> Result < () , error :: Unspecified > { parse_big_endian_and_pad_consttime (input , result) . map_err (error :: erase :: < LenMismatchError >) ? ; verify_limbs_less_than_limbs_leak_bit (result , max_exclusive) ? ; if allow_zero != AllowZero :: Yes { if limbs_are_zero (result) . leak () { return Err (error :: Unspecified) ; } } Ok (()) }
};
}
