// Generated macro for impl_99 (impl)
macro_rules! Depcrate_mapimpl_99 {
() => {
// Module: crate::map
// Provides: {"impl_99"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < K , V , const N : usize > From < [(K , V) ; N] > for IndexMap < K , V , RandomState > where K : Hash + Eq , { # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let map1 = IndexMap::from([(1, 2), (3, 4)]);"] # [doc = " let map2: IndexMap<_, _> = [(1, 2), (3, 4)].into();"] # [doc = " assert_eq!(map1, map2);"] # [doc = " ```"] fn from (arr : [(K , V) ; N]) -> Self { Self :: from_iter (arr) } }
};
}
