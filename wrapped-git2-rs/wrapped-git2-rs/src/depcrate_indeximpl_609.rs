// Generated macro for impl_609 (impl)
macro_rules! Depcrate_indeximpl_609 {
() => {
// Module: crate::index
// Provides: {"impl_609"}
// Dependencies: {}
impl < 'index > Binding for IndexConflicts < 'index > { type Raw = * mut raw :: git_index_conflict_iterator ; unsafe fn from_raw (raw : * mut raw :: git_index_conflict_iterator) -> IndexConflicts < 'index > { IndexConflicts { conflict_iter : raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_index_conflict_iterator { self . conflict_iter } }
};
}
