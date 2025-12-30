// Generated macro for impl_913 (impl)
macro_rules! Depcrate_rebaseimpl_913 {
() => {
// Module: crate::rebase
// Provides: {"impl_913"}
// Dependencies: {}
impl < 'rebase > Binding for RebaseOperation < 'rebase > { type Raw = * const raw :: git_rebase_operation ; unsafe fn from_raw (raw : * const raw :: git_rebase_operation) -> RebaseOperation < 'rebase > { RebaseOperation { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_rebase_operation { self . raw } }
};
}
