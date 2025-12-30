// Generated macro for Frames (struct)
macro_rules! Depcrate_animationFrames {
() => {
// Module: crate::animation
// Provides: {"Frames"}
// Dependencies: {}
# [doc = " An implementation dependent iterator, reading the frames as requested"] pub struct Frames < 'a > { iterator : Box < dyn Iterator < Item = ImageResult < Frame > > + 'a > , }
};
}
