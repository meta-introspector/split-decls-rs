macro_rules! fmaf16 {
    () => {
        # [allow (unused)] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn fmaf16 (_x : f16 , _y : f16 , _z : f16) -> f16 { unimplemented ! () }
    };
}

fmaf16!();