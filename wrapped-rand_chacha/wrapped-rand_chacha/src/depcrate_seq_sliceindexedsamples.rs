// Generated macro for IndexedSamples (struct)
macro_rules! Depcrate_seq_sliceIndexedSamples {
() => {
// Module: crate::seq::slice
// Provides: {"IndexedSamples"}
// Dependencies: {}
# [doc = " An iterator over multiple slice elements."] # [doc = ""] # [doc = " This struct is created by"] # [doc = " [`IndexedRandom::sample`](trait.IndexedRandom.html#tymethod.sample)."] # [cfg (feature = "alloc")] # [derive (Debug)] pub struct IndexedSamples < 'a , S : ? Sized + 'a , T : 'a > { slice : & 'a S , _phantom : core :: marker :: PhantomData < T > , indices : index :: IndexVecIntoIter , }
};
}
