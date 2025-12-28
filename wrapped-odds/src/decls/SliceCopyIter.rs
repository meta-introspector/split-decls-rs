macro_rules! SliceCopyIter {
    () => {
        # [doc = " Slice (contiguous data) iterator."] # [doc = ""] # [doc = " Iterator element type is `T` (by value)."] # [doc = " This iterator exists mainly to have the constructor from a pair"] # [doc = " of raw pointers available, which the libcore slice iterator does not allow."] # [doc = ""] # [doc = " Implementation note: Aliasing/optimization issues disappear if we use"] # [doc = " non-pointer iterator element type, so we use `T`. (The libcore slice"] # [doc = " iterator has `assume` and other tools available to combat it)."] # [doc = ""] # [doc = " `T` must not be a zero sized type."] # [derive (Debug)] pub struct SliceCopyIter < 'a , T : 'a > { ptr : * const T , end : * const T , ty : PhantomData < & 'a T > , }
    };
}

SliceCopyIter!();