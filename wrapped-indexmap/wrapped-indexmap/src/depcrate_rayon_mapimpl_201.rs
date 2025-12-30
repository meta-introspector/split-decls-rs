// Generated macro for impl_201 (impl)
macro_rules! Depcrate_rayon_mapimpl_201 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_201"}
// Dependencies: {}
impl < K , V > Slice < K , V > where K : Send , V : Send , { # [doc = " Return a parallel iterator over mutable references to the the values of the map slice."] # [doc = ""] # [doc = " While parallel iterators can process items in any order, their relative order"] # [doc = " in the slice is still preserved for operations like `reduce` and `collect`."] pub fn par_values_mut (& mut self) -> ParValuesMut < '_ , K , V > { ParValuesMut { entries : & mut self . entries , } } }
};
}
