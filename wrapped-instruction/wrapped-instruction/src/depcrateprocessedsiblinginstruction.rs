// Generated macro for ProcessedSiblingInstruction (struct)
macro_rules! DepcrateProcessedSiblingInstruction {
() => {
// Module: crate
// Provides: {"ProcessedSiblingInstruction"}
// Dependencies: {}
# [doc = " Use to query and convey information about the sibling instruction components"] # [doc = " when calling the `sol_get_processed_sibling_instruction` syscall."] # [repr (C)] # [derive (Default , Debug , Clone , Copy , Eq , PartialEq)] pub struct ProcessedSiblingInstruction { # [doc = " Length of the instruction data"] pub data_len : u64 , # [doc = " Number of AccountMeta structures"] pub accounts_len : u64 , }
};
}
