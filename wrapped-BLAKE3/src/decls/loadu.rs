macro_rules! loadu {
    () => {
        # [inline (always)] unsafe fn loadu (src : * const u8) -> v128 { unsafe { v128_load (src as * const v128) } }
    };
}

loadu!();