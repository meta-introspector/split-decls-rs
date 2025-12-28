macro_rules! BitMask {
    () => {
        # [doc = " A bit mask which contains the result of a `Match` operation on a `Group` and"] # [doc = " allows iterating through them."] # [doc = ""] # [doc = " The bit mask is arranged so that low-order bits represent lower memory"] # [doc = " addresses for group match results."] # [doc = ""] # [doc = " For implementation reasons, the bits in the set may be sparsely packed with"] # [doc = " groups of 8 bits representing one element. If any of these bits are non-zero"] # [doc = " then this element is considered to true in the mask. If this is the"] # [doc = " case, `BITMASK_STRIDE` will be 8 to indicate a divide-by-8 should be"] # [doc = " performed on counts/indices to normalize this difference. `BITMASK_MASK` is"] # [doc = " similarly a mask of all the actually-used bits."] # [doc = ""] # [doc = " To iterate over a bit mask, it must be converted to a form where only 1 bit"] # [doc = " is set per element. This is done by applying `BITMASK_ITER_MASK` on the"] # [doc = " mask bits."] # [derive (Copy , Clone)] pub (crate) struct BitMask (pub (crate) BitMaskWord) ;
    };
}

BitMask!()