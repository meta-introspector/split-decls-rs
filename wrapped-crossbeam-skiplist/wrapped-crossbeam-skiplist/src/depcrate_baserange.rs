// Generated macro for Range (struct)
macro_rules! Depcrate_baseRange {
() => {
// Module: crate::base
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An iterator over a subset of entries of a `SkipList`."] pub struct Range < 'a : 'g , 'g , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { parent : & 'a SkipList < K , V > , head : Option < & 'g Node < K , V > > , tail : Option < & 'g Node < K , V > > , range : R , guard : & 'g Guard , _marker : PhantomData < fn () -> Q > , }
};
}
