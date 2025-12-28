macro_rules! BoxFuture {
    () => {
        # [doc = " An owned dynamically typed [`Future`] for use in cases where you can't"] # [doc = " statically type your result or need to add some indirection."] # [doc = ""] # [doc = " This type is often created by the [`boxed`] method on [`FutureExt`]. See its documentation for more."] # [doc = ""] # [doc = " [`boxed`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed"] # [doc = " [`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html"] # [cfg (feature = "alloc")] pub type BoxFuture < 'a , T > = Pin < alloc :: boxed :: Box < dyn Future < Output = T > + Send + 'a > > ;
    };
}

BoxFuture!()