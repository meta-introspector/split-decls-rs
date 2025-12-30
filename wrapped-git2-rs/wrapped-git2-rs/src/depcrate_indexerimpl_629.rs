// Generated macro for impl_629 (impl)
macro_rules! Depcrate_indexerimpl_629 {
() => {
// Module: crate::indexer
// Provides: {"impl_629"}
// Dependencies: {}
impl < 'a > Binding for Progress < 'a > { type Raw = * const raw :: git_indexer_progress ; unsafe fn from_raw (raw : * const raw :: git_indexer_progress) -> Progress < 'a > { Progress { raw : ProgressState :: Borrowed (raw) , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_indexer_progress { match self . raw { ProgressState :: Borrowed (raw) => raw , ProgressState :: Owned (ref raw) => raw as * const _ , } } }
};
}
