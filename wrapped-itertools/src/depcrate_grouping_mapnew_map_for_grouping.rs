// Generated macro for new_map_for_grouping (function)
macro_rules! Depcrate_grouping_mapnew_map_for_grouping {
() => {
// Module: crate::grouping_map
// Provides: {"new_map_for_grouping"}
// Dependencies: {}
pub (crate) fn new_map_for_grouping < K , I : Iterator , F : FnMut (& I :: Item) -> K > (iter : I , key_mapper : F ,) -> MapForGrouping < I , F > { MapSpecialCase { iter , f : GroupingMapFn (key_mapper) , } }
};
}
