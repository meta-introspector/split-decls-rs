macro_rules! deps {
    () => {
        One!();
    };
}

macro_rules! count_raw {
    () => {
        deps!();
        # [doc = " Count all matching bytes, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::count_raw`."] # [inline] unsafe fn count_raw (needle : u8 , start : * const u8 , end : * const u8) -> usize { # [cfg (target_arch = "x86_64")] { crate :: arch :: x86_64 :: memchr :: count_raw (needle , start , end) } # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] { crate :: arch :: wasm32 :: memchr :: count_raw (needle , start , end) } # [cfg (target_arch = "aarch64")] { crate :: arch :: aarch64 :: memchr :: count_raw (needle , start , end) } # [cfg (not (any (target_arch = "x86_64" , all (target_arch = "wasm32" , target_feature = "simd128") , target_arch = "aarch64")))] { crate :: arch :: all :: memchr :: One :: new (needle) . count_raw (start , end) } }
    };
}

count_raw!();