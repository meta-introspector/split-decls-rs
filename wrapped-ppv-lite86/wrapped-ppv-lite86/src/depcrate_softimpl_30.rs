// Generated macro for impl_30 (impl)
macro_rules! Depcrate_softimpl_30 {
() => {
// Module: crate::soft
// Provides: {"impl_30"}
// Dependencies: {}
impl < W : Copy , G > Vec2 < W > for x2 < W , G > { # [inline (always)] fn extract (self , i : u32) -> W { self . 0 [i as usize] } # [inline (always)] fn insert (mut self , w : W , i : u32) -> Self { self . 0 [i as usize] = w ; self } }
};
}
