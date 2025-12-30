// Generated macro for ComputeBudgetInstruction (enum)
macro_rules! DepcrateComputeBudgetInstruction {
() => {
// Module: crate
// Provides: {"ComputeBudgetInstruction"}
// Dependencies: {}
# [doc = " Compute Budget Instructions"] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample , solana_frozen_abi_macro :: AbiEnumVisitor))] # [cfg_attr (feature = "borsh" , derive (BorshSerialize , BorshDeserialize))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , Debug , PartialEq , Eq)] pub enum ComputeBudgetInstruction { Unused , # [doc = " Request a specific transaction-wide program heap region size in bytes."] # [doc = " The value requested must be a multiple of 1024. This new heap region"] # [doc = " size applies to each program executed in the transaction, including all"] # [doc = " calls to CPIs."] RequestHeapFrame (u32) , # [doc = " Set a specific compute unit limit that the transaction is allowed to consume."] SetComputeUnitLimit (u32) , # [doc = " Set a compute unit price in \"micro-lamports\" to pay a higher transaction"] # [doc = " fee for higher transaction prioritization."] SetComputeUnitPrice (u64) , # [doc = " Set a specific transaction-wide account data size limit, in bytes, is allowed to load."] SetLoadedAccountsDataSizeLimit (u32) , }
};
}
