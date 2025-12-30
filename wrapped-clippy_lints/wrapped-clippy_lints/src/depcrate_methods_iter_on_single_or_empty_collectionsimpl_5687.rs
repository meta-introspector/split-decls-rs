// Generated macro for impl_5687 (impl)
macro_rules! Depcrate_methods_iter_on_single_or_empty_collectionsimpl_5687 {
() => {
// Module: crate::methods::iter_on_single_or_empty_collections
// Provides: {"impl_5687"}
// Dependencies: {}
impl IterType { fn ref_prefix (& self) -> & 'static str { match self { Self :: Iter => "&" , Self :: IterMut => "&mut " , Self :: IntoIter => "" , } } }
};
}
