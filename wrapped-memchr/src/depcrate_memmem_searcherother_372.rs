// Generated macro for other_372 (other)
macro_rules! Depcrate_memmem_searcherother_372 {
() => {
// Module: crate::memmem::searcher
// Provides: {"other_372"}
// Dependencies: {}
# [doc = " A union indicating one of several possible substring search implementations"] # [doc = " that are in active use."] # [doc = ""] # [doc = " This union should only be read by one of the functions prefixed with"] # [doc = " `searcher_kind_`. Namely, the correct function is meant to be paired with"] # [doc = " the union by the caller, such that the function always reads from the"] # [doc = " designated union field."] # [derive (Clone , Copy)] union SearcherKind { empty : () , one_byte : u8 , two_way : twoway :: Finder , two_way_with_prefilter : TwoWayWithPrefilter , # [cfg (all (target_arch = "x86_64" , target_feature = "sse2"))] sse2 : crate :: arch :: x86_64 :: sse2 :: packedpair :: Finder , # [cfg (all (target_arch = "x86_64" , target_feature = "sse2"))] avx2 : crate :: arch :: x86_64 :: avx2 :: packedpair :: Finder , # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] simd128 : crate :: arch :: wasm32 :: simd128 :: packedpair :: Finder , # [cfg (target_arch = "aarch64")] neon : crate :: arch :: aarch64 :: neon :: packedpair :: Finder , }
};
}
