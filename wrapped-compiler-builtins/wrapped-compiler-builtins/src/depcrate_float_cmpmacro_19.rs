// Generated macro for macro_19 (macro)
macro_rules! Depcrate_float_cmpmacro_19 {
() => {
// Module: crate::float::cmp
// Provides: {"macro_19"}
// Dependencies: {}
# [cfg (f16_enabled)] intrinsics ! { pub extern "C" fn __lehf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } pub extern "C" fn __gehf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_ge_abi () } pub extern "C" fn __unordhf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { unord (a , b) as crate :: float :: cmp :: CmpResult } pub extern "C" fn __eqhf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } pub extern "C" fn __lthf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } pub extern "C" fn __nehf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_le_abi () } pub extern "C" fn __gthf2 (a : f16 , b : f16) -> crate :: float :: cmp :: CmpResult { cmp (a , b) . to_ge_abi () } }
};
}
