macro_rules! deps {
    () => {
        ChunkedBitSet!();
        DenseBitSet!();
    };
}

macro_rules! MixedBitSet {
    () => {
        deps!();
        # [doc = " A bitset with a mixed representation, using `DenseBitSet` for small and"] # [doc = " medium bitsets, and `ChunkedBitSet` for large bitsets, i.e. those with"] # [doc = " enough bits for at least two chunks. This is a good choice for many bitsets"] # [doc = " that can have large domain sizes (e.g. 5000+)."] # [doc = ""] # [doc = " `T` is an index type, typically a newtyped `usize` wrapper, but it can also"] # [doc = " just be `usize`."] # [doc = ""] # [doc = " All operations that involve an element will panic if the element is equal"] # [doc = " to or greater than the domain size. All operations that involve two bitsets"] # [doc = " will panic if the bitsets have differing domain sizes."] # [derive (PartialEq , Eq)] pub enum MixedBitSet < T > { Small (DenseBitSet < T >) , Large (ChunkedBitSet < T >) , }
    };
}

MixedBitSet!()