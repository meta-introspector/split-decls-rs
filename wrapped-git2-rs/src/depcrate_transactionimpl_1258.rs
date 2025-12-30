// Generated macro for impl_1258 (impl)
macro_rules! Depcrate_transactionimpl_1258 {
() => {
// Module: crate::transaction
// Provides: {"impl_1258"}
// Dependencies: {}
impl < 'repo > Binding for Transaction < 'repo > { type Raw = * mut raw :: git_transaction ; unsafe fn from_raw (ptr : * mut raw :: git_transaction) -> Transaction < 'repo > { Transaction { raw : ptr , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_transaction { self . raw } }
};
}
