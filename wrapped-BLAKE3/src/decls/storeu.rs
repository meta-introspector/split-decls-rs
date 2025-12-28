macro_rules! storeu {
    () => {
        # [inline (always)] unsafe fn storeu (src : v128 , dest : * mut u8) { unsafe { v128_store (dest as * mut v128 , src) } }
    };
}

storeu!();