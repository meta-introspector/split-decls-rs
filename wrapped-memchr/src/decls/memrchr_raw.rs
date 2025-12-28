macro_rules! deps {
    () => {
        One!();
    };
}

macro_rules! memrchr_raw {
    () => {
        deps!();
        # [doc = " memrchr, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::rfind_raw`."] # [inline] unsafe fn memrchr_raw (needle : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { # [cfg (target_arch = "x86_64")] { crate :: arch :: x86_64 :: memchr :: memrchr_raw (needle , start , end) } # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] { crate :: arch :: wasm32 :: memchr :: memrchr_raw (needle , start , end) } # [cfg (target_arch = "aarch64")] { crate :: arch :: aarch64 :: memchr :: memrchr_raw (needle , start , end) } # [cfg (not (any (target_arch = "x86_64" , all (target_arch = "wasm32" , target_feature = "simd128") , target_arch = "aarch64")))] { crate :: arch :: all :: memchr :: One :: new (needle) . rfind_raw (start , end) } }
    };
}

memrchr_raw!();