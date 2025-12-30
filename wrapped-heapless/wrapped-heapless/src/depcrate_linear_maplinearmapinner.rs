// Generated macro for LinearMapInner (struct)
macro_rules! Depcrate_linear_mapLinearMapInner {
() => {
// Module: crate::linear_map
// Provides: {"LinearMapInner"}
// Dependencies: {}
# [doc = " Base struct for [`LinearMap`] and [`LinearMapView`]"] # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "S: Zeroize, K: Zeroize, V: Zeroize"))] pub struct LinearMapInner < K , V , S : LinearMapStorage < K , V > + ? Sized > { pub (crate) buffer : VecInner < (K , V) , usize , S > , }
};
}
