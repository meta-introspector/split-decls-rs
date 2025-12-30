// Generated macro for impl_5626 (impl)
macro_rules! Depcrate_methods_iter_on_single_or_empty_collectionsimpl_5626 {
() => {
// Module: crate::methods::iter_on_single_or_empty_collections
// Provides: {"impl_5626"}
// Dependencies: {}
impl IterType { fn ref_prefix (& self) -> & 'static str { match self { Self :: Iter => "&" , Self :: IterMut => "&mut " , Self :: IntoIter => "" , } } }
};
}
