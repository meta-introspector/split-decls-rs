// Generated macro for LocalBoxFuture (type)
macro_rules! Depcrate_futureLocalBoxFuture {
() => {
// Module: crate::future
// Provides: {"LocalBoxFuture"}
// Dependencies: {}
# [doc = " `BoxFuture`, but without the `Send` requirement."] # [doc = ""] # [doc = " This type is often created by the [`boxed_local`] method on [`FutureExt`]. See its documentation for more."] # [doc = ""] # [doc = " [`boxed_local`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed_local"] # [doc = " [`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html"] # [cfg (feature = "alloc")] pub type LocalBoxFuture < 'a , T > = Pin < alloc :: boxed :: Box < dyn Future < Output = T > + 'a > > ;
};
}
