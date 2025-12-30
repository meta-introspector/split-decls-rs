// Generated macro for HKDF_INFO_DEFAULT_CAPACITY_LEN (const)
macro_rules! Depcrate_hkdfHKDF_INFO_DEFAULT_CAPACITY_LEN {
() => {
// Module: crate::hkdf
// Provides: {"HKDF_INFO_DEFAULT_CAPACITY_LEN"}
// Dependencies: {}
# [doc = " General Info length's for HKDF don't normally exceed 256 bits."] # [doc = " We set the default capacity to a value larger than should be needed"] # [doc = " so that the value passed to |`HKDF_expand`| is only allocated once."] const HKDF_INFO_DEFAULT_CAPACITY_LEN : usize = 300 ;
};
}
