macro_rules! deps {
    () => {
        ArcInner!();
    };
}

macro_rules! Arc {
    () => {
        deps!();
        # [doc = " An atomically reference counted shared pointer"] # [doc = ""] # [doc = " See the documentation for [`Arc`] in the standard library. Unlike the"] # [doc = " standard library `Arc`, this `Arc` does not support weak reference counting."] # [doc = ""] # [doc = " [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html"] # [repr (transparent)] pub (crate) struct Arc < T : ? Sized > { pub (crate) p : ptr :: NonNull < ArcInner < T > > , pub (crate) phantom : PhantomData < T > , }
    };
}

Arc!();