// Generated macro for impl_186 (impl)
macro_rules! Depcrate_bits64_pagingimpl_186 {
() => {
// Module: crate::bits64::paging
// Provides: {"impl_186"}
// Dependencies: {}
impl ops :: Sub < usize > for PAddr { type Output = PAddr ; fn sub (self , rhs : usize) -> Self :: Output { PAddr :: from (self . 0 - rhs as u64) } }
};
}
