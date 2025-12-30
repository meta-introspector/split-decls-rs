// Generated macro for deserializer_from_reader_with_buffer_and_recursion_limit (function)
macro_rules! Depcrate_dedeserializer_from_reader_with_buffer_and_recursion_limit {
() => {
// Module: crate::de
// Provides: {"deserializer_from_reader_with_buffer_and_recursion_limit"}
// Dependencies: {}
# [doc = " Returns a deserializer with a specified scratch buffer"] # [doc = " amd maximum recursion limit. Inputs that are nested beyond the specified limit"] # [doc = " will result in [`Error::RecursionLimitExceeded`] ."] # [doc = ""] # [doc = " Set a high recursion limit at your own risk (of stack exhaustion)!"] # [inline] pub fn deserializer_from_reader_with_buffer_and_recursion_limit < R : Read > (reader : R , scratch_buffer : & mut [u8] , recurse_limit : usize ,) -> Deserializer < '_ , R > where R :: Error : core :: fmt :: Debug , { Deserializer { decoder : reader . into () , scratch : scratch_buffer , recurse : recurse_limit , } }
};
}
