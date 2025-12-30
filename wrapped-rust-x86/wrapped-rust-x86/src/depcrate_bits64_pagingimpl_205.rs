// Generated macro for impl_205 (impl)
macro_rules! Depcrate_bits64_pagingimpl_205 {
() => {
// Module: crate::bits64::paging
// Provides: {"impl_205"}
// Dependencies: {}
# [cfg (feature = "unstable")] impl Step for IOAddr { fn steps_between (start : & Self , end : & Self) -> Option < usize > { < u64 as Step > :: steps_between (& start . 0 , & end . 0) } fn forward_checked (start : Self , count : usize) -> Option < Self > { < u64 as Step > :: forward_checked (start . 0 , count) . map (| v | IOAddr (v)) } fn backward_checked (start : Self , count : usize) -> Option < Self > { < u64 as Step > :: backward_checked (start . 0 , count) . map (| v | IOAddr (v)) } }
};
}
