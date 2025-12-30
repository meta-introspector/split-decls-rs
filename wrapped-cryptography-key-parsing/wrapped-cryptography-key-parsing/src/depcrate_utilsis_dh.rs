// Generated macro for is_dh (function)
macro_rules! Depcrate_utilsis_dh {
() => {
// Module: crate::utils
// Provides: {"is_dh"}
// Dependencies: {}
pub (crate) fn is_dh (id : openssl :: pkey :: Id) -> bool { cfg_if :: cfg_if ! { if # [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] { id == openssl :: pkey :: Id :: DH || id == openssl :: pkey :: Id :: DHX } else { id == openssl :: pkey :: Id :: DH } } }
};
}
