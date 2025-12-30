// Generated macro for FeeStructure (struct)
macro_rules! DepcrateFeeStructure {
() => {
// Module: crate
// Provides: {"FeeStructure"}
// Dependencies: {}
# [doc = " Information used to calculate fees"] # [derive (Debug , Clone , Eq , PartialEq)] pub struct FeeStructure { # [doc = " lamports per signature"] pub lamports_per_signature : u64 , # [doc = " lamports_per_write_lock"] pub lamports_per_write_lock : u64 , # [doc = " Compute unit fee bins"] pub compute_fee_bins : Vec < FeeBin > , }
};
}
