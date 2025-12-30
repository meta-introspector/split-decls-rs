// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl ComputeBudgetInstruction { # [doc = " Create a `ComputeBudgetInstruction::RequestHeapFrame` `Instruction`"] pub fn request_heap_frame (bytes : u32) -> Instruction { to_instruction ! (1 , bytes , u32) } # [doc = " Create a `ComputeBudgetInstruction::SetComputeUnitLimit` `Instruction`"] pub fn set_compute_unit_limit (units : u32) -> Instruction { to_instruction ! (2 , units , u32) } # [doc = " Create a `ComputeBudgetInstruction::SetComputeUnitPrice` `Instruction`"] pub fn set_compute_unit_price (micro_lamports : u64) -> Instruction { to_instruction ! (3 , micro_lamports , u64) } # [doc = " Serialize Instruction using borsh, this is only used in runtime::cost_model::tests but compilation"] # [doc = " can't be restricted as it's used across packages"] # [cfg (feature = "dev-context-only-utils")] pub fn pack (self) -> Result < Vec < u8 > , borsh :: io :: Error > { borsh :: to_vec (& self) } # [doc = " Create a `ComputeBudgetInstruction::SetLoadedAccountsDataSizeLimit` `Instruction`"] pub fn set_loaded_accounts_data_size_limit (bytes : u32) -> Instruction { to_instruction ! (4 , bytes , u32) } }
};
}
