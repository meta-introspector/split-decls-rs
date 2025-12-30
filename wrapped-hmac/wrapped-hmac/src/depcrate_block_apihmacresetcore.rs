// Generated macro for HmacResetCore (struct)
macro_rules! Depcrate_block_apiHmacResetCore {
() => {
// Module: crate::block_api
// Provides: {"HmacResetCore"}
// Dependencies: {}
# [doc = " Generic core HMAC instance, which operates over blocks."] pub struct HmacResetCore < D : EagerHash > { digest : D :: Core , opad_digest : D :: Core , ipad_digest : D :: Core , }
};
}
