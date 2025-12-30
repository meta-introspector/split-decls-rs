// Generated macro for impl_352 (impl)
macro_rules! Depcrate_dynimageimpl_352 {
() => {
// Module: crate::dynimage
// Provides: {"impl_352"}
// Dependencies: {}
impl From < ImageBuffer < LumaA < f32 > , Vec < f32 > > > for DynamicImage { fn from (image : ImageBuffer < LumaA < f32 > , Vec < f32 > >) -> Self { DynamicImage :: ImageRgba32F (image . convert ()) } }
};
}
