macro_rules! HmacCore {
    () => {
        # [doc = " Generic core HMAC instance, which operates over blocks."] pub struct HmacCore < D : EagerHash > { digest : D :: Core , opad_digest : D :: Core , }
    };
}

HmacCore!()