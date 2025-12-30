// Generated macro for load_instruction_at (function)
macro_rules! Depcrateload_instruction_at {
() => {
// Module: crate
// Provides: {"load_instruction_at"}
// Dependencies: {}
# [doc = " Load an `Instruction` in the currently executing `Transaction` at the"] # [doc = " specified index."] # [doc = ""] # [doc = " `data` is the instructions sysvar account data."] # [doc = ""] # [doc = " Unsafe because the sysvar accounts address is not checked; only used"] # [doc = " internally after such a check."] # [cfg_attr (feature = "dev-context-only-utils" , qualifiers (pub))] fn load_instruction_at (index : usize , data : & [u8]) -> Result < Instruction , SanitizeError > { deserialize_instruction (index , data) }
};
}
