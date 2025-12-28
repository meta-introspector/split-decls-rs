macro_rules! deps {
    () => {
        Two!();
    };
}

macro_rules! memchr2_raw {
    () => {
        deps!();
        # [doc = " memchr2, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Two::find_raw`."] # [inline] unsafe fn memchr2_raw (needle1 : u8 , needle2 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { # [cfg (target_arch = "x86_64")] { crate :: arch :: x86_64 :: memchr :: memchr2_raw (needle1 , needle2 , start , end) } # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] { crate :: arch :: wasm32 :: memchr :: memchr2_raw (needle1 , needle2 , start , end) } # [cfg (target_arch = "aarch64")] { crate :: arch :: aarch64 :: memchr :: memchr2_raw (needle1 , needle2 , start , end) } # [cfg (not (any (target_arch = "x86_64" , all (target_arch = "wasm32" , target_feature = "simd128") , target_arch = "aarch64")))] { crate :: arch :: all :: memchr :: Two :: new (needle1 , needle2) . find_raw (start , end) } }
    };
}

memchr2_raw!()