macro_rules! deps {
    () => {
        Three!();
    };
}

macro_rules! memchr3_raw {
    () => {
        deps!();
        # [doc = " memchr3, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Three::find_raw`."] # [inline] unsafe fn memchr3_raw (needle1 : u8 , needle2 : u8 , needle3 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { # [cfg (target_arch = "x86_64")] { crate :: arch :: x86_64 :: memchr :: memchr3_raw (needle1 , needle2 , needle3 , start , end ,) } # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] { crate :: arch :: wasm32 :: memchr :: memchr3_raw (needle1 , needle2 , needle3 , start , end ,) } # [cfg (target_arch = "aarch64")] { crate :: arch :: aarch64 :: memchr :: memchr3_raw (needle1 , needle2 , needle3 , start , end ,) } # [cfg (not (any (target_arch = "x86_64" , all (target_arch = "wasm32" , target_feature = "simd128") , target_arch = "aarch64")))] { crate :: arch :: all :: memchr :: Three :: new (needle1 , needle2 , needle3) . find_raw (start , end) } }
    };
}

memchr3_raw!();