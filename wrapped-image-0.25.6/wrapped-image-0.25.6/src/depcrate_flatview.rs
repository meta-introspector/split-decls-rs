// Generated macro for View (struct)
macro_rules! Depcrate_flatView {
() => {
// Module: crate::flat
// Provides: {"View"}
// Dependencies: {}
# [doc = " A flat buffer that can be used as an image view."] # [doc = ""] # [doc = " This is a nearly trivial wrapper around a buffer but at least sanitizes by checking the buffer"] # [doc = " length first and constraining the pixel type."] # [doc = ""] # [doc = " Note that this does not eliminate panics as the `AsRef<[T]>` implementation of `Buffer` may be"] # [doc = " unreliable, i.e. return different buffers at different times. This of course is a non-issue for"] # [doc = " all common collections where the bounds check once must be enough."] # [doc = ""] # [doc = " # Inner invariants"] # [doc = ""] # [doc = " * For all indices inside bounds, the corresponding index is valid in the buffer"] # [doc = " * `P::channel_count()` agrees with `self.inner.layout.channels`"] # [derive (Clone , Debug)] pub struct View < Buffer , P : Pixel > where Buffer : AsRef < [P :: Subpixel] > , { inner : FlatSamples < Buffer > , phantom : PhantomData < P > , }
};
}
