// Generated macro for impl_200 (impl)
macro_rules! Depcrate_rayon_mapimpl_200 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_200"}
// Dependencies: {}
impl < K , V , S > IndexMap < K , V , S > where K : Send , V : Send , { # [doc = " Return a parallel iterator over mutable references to the values of the map"] # [doc = ""] # [doc = " While parallel iterators can process items in any order, their relative order"] # [doc = " in the map is still preserved for operations like `reduce` and `collect`."] pub fn par_values_mut (& mut self) -> ParValuesMut < '_ , K , V > { ParValuesMut { entries : self . as_entries_mut () , } } }
};
}
