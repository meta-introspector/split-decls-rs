// Generated macro for impl_173 (impl)
macro_rules! Depcrate_arcimpl_173 {
() => {
// Module: crate::arc
// Provides: {"impl_173"}
// Dependencies: {}
impl < H , T > Drop for ThinArc < H , T > { # [inline] fn drop (& mut self) { let _ = Arc :: from_thin (ThinArc { ptr : self . ptr , phantom : PhantomData }) ; } }
};
}
