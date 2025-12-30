// Generated macro for impl_339 (impl)
macro_rules! Depcrate_dynimageimpl_339 {
() => {
// Module: crate::dynimage
// Provides: {"impl_339"}
// Dependencies: {}
impl Clone for DynamicImage { fn clone (& self) -> Self { dynamic_map ! (* self , ref p , DynamicImage :: from (p . clone ())) } fn clone_from (& mut self , source : & Self) { match (self , source) { (Self :: ImageLuma8 (p1) , Self :: ImageLuma8 (p2)) => p1 . clone_from (p2) , (Self :: ImageLumaA8 (p1) , Self :: ImageLumaA8 (p2)) => p1 . clone_from (p2) , (Self :: ImageRgb8 (p1) , Self :: ImageRgb8 (p2)) => p1 . clone_from (p2) , (Self :: ImageRgba8 (p1) , Self :: ImageRgba8 (p2)) => p1 . clone_from (p2) , (Self :: ImageLuma16 (p1) , Self :: ImageLuma16 (p2)) => p1 . clone_from (p2) , (Self :: ImageLumaA16 (p1) , Self :: ImageLumaA16 (p2)) => p1 . clone_from (p2) , (Self :: ImageRgb16 (p1) , Self :: ImageRgb16 (p2)) => p1 . clone_from (p2) , (Self :: ImageRgba16 (p1) , Self :: ImageRgba16 (p2)) => p1 . clone_from (p2) , (Self :: ImageRgb32F (p1) , Self :: ImageRgb32F (p2)) => p1 . clone_from (p2) , (Self :: ImageRgba32F (p1) , Self :: ImageRgba32F (p2)) => p1 . clone_from (p2) , (this , source) => * this = source . clone () , } } }
};
}
