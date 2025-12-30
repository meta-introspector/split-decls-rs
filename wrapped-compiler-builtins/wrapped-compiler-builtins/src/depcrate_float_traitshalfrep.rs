// Generated macro for HalfRep (type)
macro_rules! Depcrate_float_traitsHalfRep {
() => {
// Module: crate::float::traits
// Provides: {"HalfRep"}
// Dependencies: {}
# [doc = " Wrapper to extract the integer type half of the float's size"] pub type HalfRep < F > = < < F as Float > :: Int as DInt > :: H ;
};
}
