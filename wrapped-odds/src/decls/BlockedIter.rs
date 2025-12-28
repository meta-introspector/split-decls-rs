macro_rules! BlockedIter {
    () => {
        # [doc = " An iterator that yields blocks out of the underlying data's range."] # [doc = ""] # [doc = " A block is a fixed size array of `T`, and each iteration yields a"] # [doc = " reference to the next block."] # [doc = ""] # [doc = " See also the tail methods that provide access to the rest of the elements"] # [doc = " that did not go up evenly into a block."] # [derive (Debug)] pub struct BlockedIter < 'a , B : 'a , T : 'a > { ptr : * const T , end : * const T , ty1 : PhantomData < & 'a T > , ty2 : PhantomData < & 'a B > , }
    };
}

BlockedIter!()