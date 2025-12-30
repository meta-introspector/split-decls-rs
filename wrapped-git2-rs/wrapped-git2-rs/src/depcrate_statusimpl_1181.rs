// Generated macro for impl_1181 (impl)
macro_rules! Depcrate_statusimpl_1181 {
() => {
// Module: crate::status
// Provides: {"impl_1181"}
// Dependencies: {}
impl < 'statuses > Binding for StatusEntry < 'statuses > { type Raw = * const raw :: git_status_entry ; unsafe fn from_raw (raw : * const raw :: git_status_entry) -> StatusEntry < 'statuses > { StatusEntry { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_status_entry { self . raw } }
};
}
