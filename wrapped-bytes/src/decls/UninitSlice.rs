macro_rules! UninitSlice {
    () => {
        # [doc = " Uninitialized byte slice."] # [doc = ""] # [doc = " Returned by `BufMut::chunk_mut()`, the referenced byte slice may be"] # [doc = " uninitialized. The wrapper provides safe access without introducing"] # [doc = " undefined behavior."] # [doc = ""] # [doc = " The safety invariants of this wrapper are:"] # [doc = ""] # [doc = "  1. Reading from an `UninitSlice` is undefined behavior."] # [doc = "  2. Writing uninitialized bytes to an `UninitSlice` is undefined behavior."] # [doc = ""] # [doc = " The difference between `&mut UninitSlice` and `&mut [MaybeUninit<u8>]` is"] # [doc = " that it is possible in safe code to write uninitialized bytes to an"] # [doc = " `&mut [MaybeUninit<u8>]`, which this type prohibits."] # [repr (transparent)] pub struct UninitSlice ([MaybeUninit < u8 >]) ;
    };
}

UninitSlice!();