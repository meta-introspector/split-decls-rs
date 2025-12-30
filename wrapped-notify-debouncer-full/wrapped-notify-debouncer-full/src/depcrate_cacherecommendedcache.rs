// Generated macro for RecommendedCache (type)
macro_rules! Depcrate_cacheRecommendedCache {
() => {
// Module: crate::cache
// Provides: {"RecommendedCache"}
// Dependencies: {}
# [doc = " The recommended file ID cache implementation for the current platform"] # [cfg (not (any (target_os = "linux" , target_os = "android")))] pub type RecommendedCache = FileIdMap ;
};
}
