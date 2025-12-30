// Generated macro for impl_351 (impl)
macro_rules! Depcrate_dynimageimpl_351 {
() => {
// Module: crate::dynimage
// Provides: {"impl_351"}
// Dependencies: {}
impl From < ImageBuffer < Luma < f32 > , Vec < f32 > > > for DynamicImage { fn from (image : ImageBuffer < Luma < f32 > , Vec < f32 > >) -> Self { DynamicImage :: ImageRgb32F (image . convert ()) } }
};
}
