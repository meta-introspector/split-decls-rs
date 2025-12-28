macro_rules! deps {
    () => {
        Stream!();
        BoxStream!();
    };
}

macro_rules! LocalBoxStream {
    () => {
        deps!();
        # [doc = " `BoxStream`, but without the `Send` requirement."] # [doc = ""] # [doc = " This type is often created by the [`boxed_local`] method on [`StreamExt`]. See its documentation for more."] # [doc = ""] # [doc = " [`boxed_local`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed_local"] # [doc = " [`StreamExt`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html"] # [cfg (feature = "alloc")] pub type LocalBoxStream < 'a , T > = Pin < alloc :: boxed :: Box < dyn Stream < Item = T > + 'a > > ;
    };
}

LocalBoxStream!();