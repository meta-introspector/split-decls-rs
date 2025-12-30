// Generated macro for unique_by (function)
macro_rules! Depcrate_unique_implunique_by {
() => {
// Module: crate::unique_impl
// Provides: {"unique_by"}
// Dependencies: {}
# [doc = " Create a new `UniqueBy` iterator."] pub fn unique_by < I , V , F > (iter : I , f : F) -> UniqueBy < I , V , F > where V : Eq + Hash , F : FnMut (& I :: Item) -> V , I : Iterator , { UniqueBy { iter , used : HashMap :: new () , f , } }
};
}
