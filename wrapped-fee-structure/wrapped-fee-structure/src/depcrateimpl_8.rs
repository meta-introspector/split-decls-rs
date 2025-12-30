// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl FeeStructure { pub fn get_max_fee (& self , num_signatures : u64 , num_write_locks : u64) -> u64 { num_signatures . saturating_mul (self . lamports_per_signature) . saturating_add (num_write_locks . saturating_mul (self . lamports_per_write_lock)) . saturating_add (self . compute_fee_bins . last () . map (| bin | bin . fee) . unwrap_or_default () ,) } pub fn calculate_memory_usage_cost (loaded_accounts_data_size_limit : u32 , heap_cost : u64 ,) -> u64 { (loaded_accounts_data_size_limit as u64) . saturating_add (ACCOUNT_DATA_COST_PAGE_SIZE . saturating_sub (1)) . saturating_div (ACCOUNT_DATA_COST_PAGE_SIZE) . saturating_mul (heap_cost) } }
};
}
