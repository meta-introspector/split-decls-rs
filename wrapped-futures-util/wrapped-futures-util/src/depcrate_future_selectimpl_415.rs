// Generated macro for impl_415 (impl)
macro_rules! Depcrate_future_selectimpl_415 {
() => {
// Module: crate::future::select
// Provides: {"impl_415"}
// Dependencies: {}
impl < A , B > FusedFuture for Select < A , B > where A : Future + Unpin , B : Future + Unpin , { fn is_terminated (& self) -> bool { self . inner . is_none () } }
};
}
