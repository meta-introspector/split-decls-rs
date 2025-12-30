// Generated macro for impl_43 (impl)
macro_rules! Depcrate_bits32_pagingimpl_43 {
() => {
// Module: crate::bits32::paging
// Provides: {"impl_43"}
// Dependencies: {}
impl ops :: Sub < usize > for PAddr { type Output = PAddr ; fn sub (self , rhs : usize) -> Self :: Output { PAddr :: from (self . 0 - rhs as u32) } }
};
}
