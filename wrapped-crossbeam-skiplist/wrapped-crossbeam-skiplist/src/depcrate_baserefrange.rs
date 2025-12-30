// Generated macro for RefRange (struct)
macro_rules! Depcrate_baseRefRange {
() => {
// Module: crate::base
// Provides: {"RefRange"}
// Dependencies: {}
# [doc = " An iterator over reference-counted subset of entries of a `SkipList`."] pub struct RefRange < 'a , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { parent : & 'a SkipList < K , V > , pub (crate) head : Option < RefEntry < 'a , K , V > > , pub (crate) tail : Option < RefEntry < 'a , K , V > > , pub (crate) range : R , _marker : PhantomData < fn () -> Q > , }
};
}
