// Generated macro for impl_237 (impl)
macro_rules! Depcrate_bits64_pagingimpl_237 {
() => {
// Module: crate::bits64::paging
// Provides: {"impl_237"}
// Dependencies: {}
# [cfg (feature = "unstable")] impl Step for VAddr { fn steps_between (start : & Self , end : & Self) -> Option < usize > { < u64 as Step > :: steps_between (& start . 0 , & end . 0) } fn forward_checked (start : Self , count : usize) -> Option < Self > { < u64 as Step > :: forward_checked (start . 0 , count) . map (| v | VAddr (v)) } fn backward_checked (start : Self , count : usize) -> Option < Self > { < u64 as Step > :: backward_checked (start . 0 , count) . map (| v | VAddr (v)) } }
};
}
