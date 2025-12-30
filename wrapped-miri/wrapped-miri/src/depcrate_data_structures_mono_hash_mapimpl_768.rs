// Generated macro for impl_768 (impl)
macro_rules! Depcrate_data_structures_mono_hash_mapimpl_768 {
() => {
// Module: crate::data_structures::mono_hash_map
// Provides: {"impl_768"}
// Dependencies: {}
impl < K : Hash + Eq , V > MonoHashMap < K , V > { # [doc = " This function exists for priroda to be able to iterate over all evaluator memory."] # [doc = ""] # [doc = " The function is somewhat roundabout with the closure argument because internally the"] # [doc = " `MonoHashMap` uses a `RefCell`. When iterating over the `FxHashMap` inside the `RefCell`,"] # [doc = " we need to keep a borrow to the `FxHashMap` inside the iterator. The borrow is only alive"] # [doc = " as long as the `Ref` returned by `RefCell::borrow()` is alive. So we can't return the"] # [doc = " iterator, as that would drop the `Ref`. We can't return both, as it's not possible in Rust"] # [doc = " to have a struct/tuple with a field that refers to another field."] pub fn iter < T > (& self , f : impl FnOnce (& mut dyn Iterator < Item = (& K , & V) >) -> T) -> T { f (& mut self . 0 . borrow () . iter () . map (| (k , v) | (k , & * * v))) } }
};
}
