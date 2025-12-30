// Generated macro for into_group_map (function)
macro_rules! Depcrate_group_mapinto_group_map {
() => {
// Module: crate::group_map
// Provides: {"into_group_map"}
// Dependencies: {}
# [doc = " Return a `HashMap` of keys mapped to a list of their corresponding values."] # [doc = ""] # [doc = " See [`.into_group_map()`](crate::Itertools::into_group_map)"] # [doc = " for more information."] pub fn into_group_map < I , K , V > (iter : I) -> HashMap < K , Vec < V > > where I : Iterator < Item = (K , V) > , K : Hash + Eq , { let mut lookup = HashMap :: < K , Vec < V > > :: new () ; iter . for_each (| (key , val) | { lookup . entry (key) . or_default () . push (val) ; }) ; lookup }
};
}
