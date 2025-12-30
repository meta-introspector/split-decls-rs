// Generated macro for ParIterMut (struct)
macro_rules! Depcrate_vector_rayonParIterMut {
() => {
// Module: crate::vector::rayon
// Provides: {"ParIterMut"}
// Dependencies: {}
# [doc = " A mutable parallel iterator for [`Vector`][Vector]."] # [doc = ""] # [doc = " [Vector]: ../struct.Vector.html"] pub struct ParIterMut < 'a , A > where A : Clone + Send + Sync , { focus : FocusMut < 'a , A > , }
};
}
