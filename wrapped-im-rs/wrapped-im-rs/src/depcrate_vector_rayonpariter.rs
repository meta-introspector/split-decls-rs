// Generated macro for ParIter (struct)
macro_rules! Depcrate_vector_rayonParIter {
() => {
// Module: crate::vector::rayon
// Provides: {"ParIter"}
// Dependencies: {}
# [doc = " A parallel iterator for [`Vector`][Vector]."] # [doc = ""] # [doc = " [Vector]: ../struct.Vector.html"] pub struct ParIter < 'a , A > where A : Clone + Send + Sync , { focus : Focus < 'a , A > , }
};
}
