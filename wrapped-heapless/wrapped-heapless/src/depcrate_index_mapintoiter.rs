// Generated macro for IntoIter (struct)
macro_rules! Depcrate_index_mapIntoIter {
() => {
// Module: crate::index_map
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the entries of an `IndexMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`IndexMap`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: IntoIterator::into_iter"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::index_map::FnvIndexMap;"] # [doc = ""] # [doc = " let mut map = FnvIndexMap::<_, _, 16>::new();"] # [doc = " map.insert(\"a\", 1).unwrap();"] # [doc = ""] # [doc = " let iter = map.into_iter();"] # [doc = " ```"] # [derive (Clone)] pub struct IntoIter < K , V , const N : usize > { entries : Vec < Bucket < K , V > , N , usize > , }
};
}
