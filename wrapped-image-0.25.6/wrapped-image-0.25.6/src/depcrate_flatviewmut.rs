// Generated macro for ViewMut (struct)
macro_rules! Depcrate_flatViewMut {
() => {
// Module: crate::flat
// Provides: {"ViewMut"}
// Dependencies: {}
# [doc = " A mutable owning version of a flat buffer."] # [doc = ""] # [doc = " While this wraps a buffer similar to `ImageBuffer`, this is mostly intended as a utility. The"] # [doc = " library endorsed normalized representation is still `ImageBuffer`. Also, the implementation of"] # [doc = " `AsMut<[P::Subpixel]>` must always yield the same buffer. Therefore there is no public way to"] # [doc = " construct this with an owning buffer."] # [doc = ""] # [doc = " # Inner invariants"] # [doc = ""] # [doc = " * For all indices inside bounds, the corresponding index is valid in the buffer"] # [doc = " * There is no aliasing of samples"] # [doc = " * The samples are packed, i.e. `self.inner.layout.sample_stride == 1`"] # [doc = " * `P::channel_count()` agrees with `self.inner.layout.channels`"] # [derive (Clone , Debug)] pub struct ViewMut < Buffer , P : Pixel > where Buffer : AsMut < [P :: Subpixel] > , { inner : FlatSamples < Buffer > , phantom : PhantomData < P > , }
};
}
