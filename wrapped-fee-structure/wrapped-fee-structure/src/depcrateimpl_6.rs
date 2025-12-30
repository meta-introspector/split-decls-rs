// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl FeeDetails { pub fn new (transaction_fee : u64 , prioritization_fee : u64) -> Self { Self { transaction_fee , prioritization_fee , } } pub fn total_fee (& self) -> u64 { self . transaction_fee . saturating_add (self . prioritization_fee) } pub fn accumulate (& mut self , fee_details : & FeeDetails) { self . transaction_fee = self . transaction_fee . saturating_add (fee_details . transaction_fee) ; self . prioritization_fee = self . prioritization_fee . saturating_add (fee_details . prioritization_fee) } pub fn transaction_fee (& self) -> u64 { self . transaction_fee } pub fn prioritization_fee (& self) -> u64 { self . prioritization_fee } }
};
}
