// Generated macro for MAX_HKDF_SALT_LEN (const)
macro_rules! Depcrate_hkdfMAX_HKDF_SALT_LEN {
() => {
// Module: crate::hkdf
// Provides: {"MAX_HKDF_SALT_LEN"}
// Dependencies: {}
# [doc = " General Salt length's for HKDF don't normally exceed 256 bits."] # [doc = " We set the limit to something tolerable, so that the Salt structure can be stack allocatable."] const MAX_HKDF_SALT_LEN : usize = 80 ;
};
}
