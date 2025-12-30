// Generated macro for limited_deserialize (function)
macro_rules! Depcratelimited_deserialize {
() => {
// Module: crate
// Provides: {"limited_deserialize"}
// Dependencies: {}
# [doc = " Deserialize with a limit based the maximum amount of data a program can expect to get."] # [doc = " This function should be used in place of direct deserialization to help prevent OOM errors"] pub fn limited_deserialize < T > (instruction_data : & [u8] , limit : u64) -> Result < T , InstructionError > where T : serde_core :: de :: DeserializeOwned , { bincode :: options () . with_limit (limit) . with_fixint_encoding () . allow_trailing_bytes () . deserialize_from (instruction_data) . map_err (| _ | InstructionError :: InvalidInstructionData) }
};
}
