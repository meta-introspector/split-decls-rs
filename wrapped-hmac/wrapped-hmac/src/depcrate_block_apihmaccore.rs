// Generated macro for HmacCore (struct)
macro_rules! Depcrate_block_apiHmacCore {
() => {
// Module: crate::block_api
// Provides: {"HmacCore"}
// Dependencies: {}
# [doc = " Generic core HMAC instance, which operates over blocks."] pub struct HmacCore < D : EagerHash > { digest : D :: Core , opad_digest : D :: Core , }
};
}
