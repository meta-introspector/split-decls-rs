// Generated macro for impl_958 (impl)
macro_rules! Depcrate_reflogimpl_958 {
() => {
// Module: crate::reflog
// Provides: {"impl_958"}
// Dependencies: {}
impl < 'reflog > Binding for ReflogEntry < 'reflog > { type Raw = * const raw :: git_reflog_entry ; unsafe fn from_raw (raw : * const raw :: git_reflog_entry) -> ReflogEntry < 'reflog > { ReflogEntry { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_reflog_entry { self . raw } }
};
}
