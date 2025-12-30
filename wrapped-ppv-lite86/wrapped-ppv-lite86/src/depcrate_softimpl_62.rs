// Generated macro for impl_62 (impl)
macro_rules! Depcrate_softimpl_62 {
() => {
// Module: crate::soft
// Provides: {"impl_62"}
// Dependencies: {}
impl < W : Copy > Vec4 < W > for x4 < W > { # [inline (always)] fn extract (self , i : u32) -> W { self . 0 [i as usize] } # [inline (always)] fn insert (mut self , w : W , i : u32) -> Self { self . 0 [i as usize] = w ; self } }
};
}
