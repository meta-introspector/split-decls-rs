macro_rules! deps {
    () => {
        Pointable!();
        Guard!();
    };
}

macro_rules! Atomic {
    () => {
        deps!();
        # [doc = " An atomic pointer that can be safely shared between threads."] # [doc = ""] # [doc = " The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused"] # [doc = " least significant bits of the address. For example, the tag for a pointer to a sized type `T`"] # [doc = " should be less than `(1 << mem::align_of::<T>().trailing_zeros())`."] # [doc = ""] # [doc = " Any method that loads the pointer must be passed a reference to a [`Guard`]."] # [doc = ""] # [doc = " Crossbeam supports dynamically sized types.  See [`Pointable`] for details."] pub struct Atomic < T : ? Sized + Pointable > { data : AtomicPtr < () > , _marker : PhantomData < * mut T > , }
    };
}

Atomic!()