macro_rules! UnalignedIter {
    () => {
        # [doc = " An iterator of `T` (by value) where each value read from a pointer"] # [doc = " that is (possibly) unaligned."] # [doc = ""] # [doc = " See also the method `.tail()`."] # [derive (Debug)] pub struct UnalignedIter < 'a , T : 'a > { ptr : * const u8 , end : * const u8 , tail_end : * const u8 , ty : PhantomData < & 'a T > , }
    };
}

UnalignedIter!()