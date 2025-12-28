macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! BoxStream {
    () => {
        deps!();
        # [doc = " An owned dynamically typed [`Stream`] for use in cases where you can't"] # [doc = " statically type your result or need to add some indirection."] # [doc = ""] # [doc = " This type is often created by the [`boxed`] method on [`StreamExt`]. See its documentation for more."] # [doc = ""] # [doc = " [`boxed`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed"] # [doc = " [`StreamExt`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html"] # [cfg (feature = "alloc")] pub type BoxStream < 'a , T > = Pin < alloc :: boxed :: Box < dyn Stream < Item = T > + Send + 'a > > ;
    };
}

BoxStream!();