// Generated macro for impl_221 (impl)
macro_rules! Depcrate_animationimpl_221 {
() => {
// Module: crate::animation
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > Frames < 'a > { # [doc = " Creates a new `Frames` from an implementation specific iterator."] # [must_use] pub fn new (iterator : Box < dyn Iterator < Item = ImageResult < Frame > > + 'a >) -> Self { Frames { iterator } } # [doc = " Steps through the iterator from the current frame until the end and pushes each frame into"] # [doc = " a `Vec`."] # [doc = " If en error is encountered that error is returned instead."] # [doc = ""] # [doc = " Note: This is equivalent to `Frames::collect::<ImageResult<Vec<Frame>>>()`"] pub fn collect_frames (self) -> ImageResult < Vec < Frame > > { self . collect () } }
};
}
