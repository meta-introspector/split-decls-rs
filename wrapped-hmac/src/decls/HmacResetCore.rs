macro_rules! HmacResetCore {
    () => {
        # [doc = " Generic core HMAC instance, which operates over blocks."] pub struct HmacResetCore < D : EagerHash > { digest : D :: Core , opad_digest : D :: Core , ipad_digest : D :: Core , }
    };
}

HmacResetCore!();