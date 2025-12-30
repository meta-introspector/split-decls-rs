// Generated macro for impl_31 (impl)
macro_rules! Depcrate_accessimpl_31 {
() => {
// Module: crate::access
// Provides: {"impl_31"}
// Dependencies: {}
impl < A , T , F > Map < A , T , F > { # [doc = " Creates a new instance."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " * `access`: Access to the bigger structure. This is usually something like `Arc<ArcSwap>`"] # [doc = "   or `&ArcSwap`. It is technically possible to use any other [`Access`] here, though, for"] # [doc = "   example to sub-delegate into even smaller structure from a [`Map`] (or generic"] # [doc = "   [`Access`])."] # [doc = " * `projection`: A function (or closure) responsible to providing a reference into the"] # [doc = "   bigger bigger structure, selecting just subset of it. In general, it is expected to be"] # [doc = "   *cheap* (like only taking reference)."] pub fn new < R > (access : A , projection : F) -> Self where F : Fn (& T) -> & R + Clone , { Map { access , projection , _t : PhantomData , } } }
};
}
