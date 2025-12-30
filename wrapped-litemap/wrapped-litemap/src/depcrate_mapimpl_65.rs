// Generated macro for impl_65 (impl)
macro_rules! Depcrate_mapimpl_65 {
() => {
// Module: crate::map
// Provides: {"impl_65"}
// Dependencies: {}
impl < K , V , S > LiteMap < K , V , S > where K : Ord , S : StoreMut < K , V > , { # [doc = " Gets the entry for the given key in the map for in-place manipulation."] pub fn entry (& mut self , key : K) -> Entry < '_ , K , V , S > { match self . values . lm_binary_search_by (| k | k . cmp (& key)) { Ok (index) => Entry :: Occupied (OccupiedEntry { map : self , index }) , Err (index) => Entry :: Vacant (VacantEntry { map : self , key , index , }) , } } }
};
}
