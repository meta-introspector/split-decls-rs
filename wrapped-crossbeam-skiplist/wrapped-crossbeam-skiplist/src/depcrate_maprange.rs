// Generated macro for Range (struct)
macro_rules! Depcrate_mapRange {
() => {
// Module: crate::map
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An iterator over a subset of entries of a `SkipMap`."] pub struct Range < 'a , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { pub (crate) inner : base :: RefRange < 'a , Q , R , K , V > , }
};
}
