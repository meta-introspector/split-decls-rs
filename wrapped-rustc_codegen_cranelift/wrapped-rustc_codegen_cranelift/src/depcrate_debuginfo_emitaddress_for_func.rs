// Generated macro for address_for_func (function)
macro_rules! Depcrate_debuginfo_emitaddress_for_func {
() => {
// Module: crate::debuginfo::emit
// Provides: {"address_for_func"}
// Dependencies: {}
pub (super) fn address_for_func (func_id : FuncId) -> Address { let symbol = func_id . as_u32 () ; assert ! (symbol & 1 << 31 == 0) ; Address :: Symbol { symbol : symbol as usize , addend : 0 } }
};
}
