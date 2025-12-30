// Generated macro for Key (struct)
macro_rules! Depcrate_hmacKey {
() => {
// Module: crate::hmac
// Provides: {"Key"}
// Dependencies: {}
# [doc = " A key to use for HMAC signing."] # [derive (Clone)] pub struct Key { inner : digest :: BlockContext , outer : digest :: BlockContext , }
};
}
