// Generated macro for deserialize_instruction_data (function)
macro_rules! Depcratedeserialize_instruction_data {
() => {
// Module: crate
// Provides: {"deserialize_instruction_data"}
// Dependencies: {}
# [allow (clippy :: arithmetic_side_effects)] # [inline (always)] unsafe fn deserialize_instruction_data < 'a > (input : * mut u8 , mut offset : usize) -> (& 'a [u8] , usize) { # [allow (clippy :: cast_ptr_alignment)] let instruction_data_len = * (input . add (offset) as * const u64) as usize ; offset += size_of :: < u64 > () ; let instruction_data = { from_raw_parts (input . add (offset) , instruction_data_len) } ; offset += instruction_data_len ; (instruction_data , offset) }
};
}
