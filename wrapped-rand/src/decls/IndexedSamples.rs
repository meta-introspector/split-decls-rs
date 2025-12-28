macro_rules! IndexedSamples {
    () => {
        # [doc = " An iterator over multiple slice elements."] # [doc = ""] # [doc = " This struct is created by"] # [doc = " [`IndexedRandom::sample`](trait.IndexedRandom.html#tymethod.sample)."] # [cfg (feature = "alloc")] # [derive (Debug)] pub struct IndexedSamples < 'a , S : ? Sized + 'a , T : 'a > { slice : & 'a S , _phantom : core :: marker :: PhantomData < T > , indices : index :: IndexVecIntoIter , }
    };
}

IndexedSamples!();