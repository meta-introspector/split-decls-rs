// Generated macro for IntoIter (struct)
macro_rules! Depcrate_linear_mapIntoIter {
() => {
// Module: crate::linear_map
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a [`LinearMap`]."] # [doc = ""] # [doc = " This struct is created by calling the [`into_iter`](LinearMap::into_iter) method on"] # [doc = " [`LinearMap`]."] pub struct IntoIter < K , V , const N : usize > where K : Eq , { inner : < Vec < (K , V) , N , usize > as IntoIterator > :: IntoIter , }
};
}
