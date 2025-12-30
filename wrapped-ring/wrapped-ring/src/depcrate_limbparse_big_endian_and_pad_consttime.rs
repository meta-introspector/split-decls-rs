// Generated macro for parse_big_endian_and_pad_consttime (function)
macro_rules! Depcrate_limbparse_big_endian_and_pad_consttime {
() => {
// Module: crate::limb
// Provides: {"parse_big_endian_and_pad_consttime"}
// Dependencies: {}
# [doc = " Parses `input` into `result`, padding `result` with zeros to its length."] # [doc = " This attempts to be constant-time with respect to the value but not with"] # [doc = " respect to the length; it is assumed that the length is public knowledge."] pub fn parse_big_endian_and_pad_consttime (input : untrusted :: Input , result : & mut [Limb] ,) -> Result < () , LenMismatchError > { let input_limbs = limbs_from_big_endian (input , 1 ..= result . len ()) ? ; result . iter_mut () . zip (input_limbs . chain (iter :: repeat (0))) . for_each (| (r , i) | * r = i) ; Ok (()) }
};
}
