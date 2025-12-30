// Generated macro for InputTooLongError (struct)
macro_rules! Depcrate_error_input_too_longInputTooLongError {
() => {
// Module: crate::error::input_too_long
// Provides: {"InputTooLongError"}
// Dependencies: {}
pub struct InputTooLongError < T = usize > { # [doc = " Note that this might not actually be the (exact) length of the input,"] # [doc = " and its units might be lost. For example, it could be any of the"] # [doc = " following:"] # [doc = ""] # [doc = "    * The length in bytes of the entire input."] # [doc = "    * The length in bytes of some *part* of the input."] # [doc = "    * A bit length."] # [doc = "    * A length in terms of \"blocks\" or other grouping of input values."] # [doc = "    * Some intermediate quantity that was used when checking the input"] # [doc = "      length."] # [doc = "    * Some arbitrary value."] # [allow (dead_code)] imprecise_input_length : T , }
};
}
