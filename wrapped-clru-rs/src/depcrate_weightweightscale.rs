// Generated macro for WeightScale (trait)
macro_rules! Depcrate_weightWeightScale {
() => {
// Module: crate::weight
// Provides: {"WeightScale"}
// Dependencies: {}
# [doc = " Trait used to retrieve the weight of a key-value pair."] pub trait WeightScale < K , V > { # [doc = " Returns the weight of a key-value pair."] fn weight (& self , key : & K , value : & V) -> usize ; }
};
}
