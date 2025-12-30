// Generated macro for address_for_data (function)
macro_rules! Depcrate_debuginfo_emitaddress_for_data {
() => {
// Module: crate::debuginfo::emit
// Provides: {"address_for_data"}
// Dependencies: {}
pub (super) fn address_for_data (data_id : DataId) -> Address { let symbol = data_id . as_u32 () ; assert ! (symbol & 1 << 31 == 0) ; Address :: Symbol { symbol : (symbol | 1 << 31) as usize , addend : 0 } }
};
}
