// Generated macro for impl_4090 (impl)
macro_rules! Depcrate_manual_clampimpl_4090 {
() => {
// Module: crate::manual_clamp
// Provides: {"impl_4090"}
// Dependencies: {}
impl < 'a > Deref for MaybeBorrowedStmtKind < 'a > { type Target = StmtKind < 'a > ; fn deref (& self) -> & Self :: Target { match self { Self :: Borrowed (t) => t , Self :: Owned (t) => t , } } }
};
}
