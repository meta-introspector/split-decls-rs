macro_rules! deps {
    () => {
        Global!();
        Allocator!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An iterator that moves out of a vector."] # [doc = ""] # [doc = " This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)"] # [doc = " (provided by the [`IntoIterator`] trait)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::vec;"] # [doc = ""] # [doc = " let v = vec![0, 1, 2];"] # [doc = " let iter: vec::IntoIter<_> = v.into_iter();"] # [doc = " ```"] pub struct IntoIter < T , A : Allocator = Global > { pub (super) buf : NonNull < T > , pub (super) phantom : PhantomData < T > , pub (super) cap : usize , pub (super) alloc : ManuallyDrop < A > , pub (super) ptr : * const T , pub (super) end : * const T , }
    };
}

IntoIter!();