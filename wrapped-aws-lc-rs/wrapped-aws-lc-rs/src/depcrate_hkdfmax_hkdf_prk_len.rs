// Generated macro for MAX_HKDF_PRK_LEN (const)
macro_rules! Depcrate_hkdfMAX_HKDF_PRK_LEN {
() => {
// Module: crate::hkdf
// Provides: {"MAX_HKDF_PRK_LEN"}
// Dependencies: {}
# [doc = " The maximum output size of a PRK computed by |`HKDF_extract`| is the maximum digest"] # [doc = " size that can be outputted by *AWS-LC*."] const MAX_HKDF_PRK_LEN : usize = digest :: MAX_OUTPUT_LEN ;
};
}
