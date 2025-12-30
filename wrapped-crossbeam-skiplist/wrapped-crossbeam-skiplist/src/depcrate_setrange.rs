// Generated macro for Range (struct)
macro_rules! Depcrate_setRange {
() => {
// Module: crate::set
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An iterator over a subset of entries of a `SkipSet`."] pub struct Range < 'a , Q , R , T > where T : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { inner : map :: Range < 'a , Q , R , T , () > , }
};
}
