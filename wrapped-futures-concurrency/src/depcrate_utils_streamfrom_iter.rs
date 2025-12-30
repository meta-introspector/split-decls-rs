// Generated macro for from_iter (function)
macro_rules! Depcrate_utils_streamfrom_iter {
() => {
// Module: crate::utils::stream
// Provides: {"from_iter"}
// Dependencies: {}
# [doc = " Converts an iterator into a stream."] pub (crate) fn from_iter < I : IntoIterator > (iter : I) -> FromIter < I :: IntoIter > { FromIter { iter : iter . into_iter () , } }
};
}
