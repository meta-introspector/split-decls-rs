// Generated macro for macro_21 (macro)
macro_rules! Depcrate_float_cmpmacro_21 {
() => {
// Module: crate::float::cmp
// Provides: {"macro_21"}
// Dependencies: {}
# [cfg (f128_enabled)] intrinsics ! { # [ppc_alias = __lekf2] pub extern "C" fn __letf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } # [ppc_alias = __gekf2] pub extern "C" fn __getf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_ge_abi () } # [ppc_alias = __unordkf2] pub extern "C" fn __unordtf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { unord (a , b) as crate :: float :: cmp :: CmpResult } # [ppc_alias = __eqkf2] pub extern "C" fn __eqtf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } # [ppc_alias = __ltkf2] pub extern "C" fn __lttf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } # [ppc_alias = __nekf2] pub extern "C" fn __netf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } # [ppc_alias = __gtkf2] pub extern "C" fn __gttf2 (a : f128 , b : f128) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_ge_abi () } }
};
}
