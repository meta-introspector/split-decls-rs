// Generated macro for into_group_map_by (function)
macro_rules! Depcrate_group_mapinto_group_map_by {
() => {
// Module: crate::group_map
// Provides: {"into_group_map_by"}
// Dependencies: {}
pub fn into_group_map_by < I , K , V , F > (iter : I , mut f : F) -> HashMap < K , Vec < V > > where I : Iterator < Item = V > , K : Hash + Eq , F : FnMut (& V) -> K , { into_group_map (iter . map (| v | (f (& v) , v))) }
};
}
