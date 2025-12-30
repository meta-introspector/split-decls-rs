// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_statusimpl_1173 {
() => {
// Module: crate::status
// Provides: {"impl_1173"}
// Dependencies: {}
impl < 'repo > Binding for Statuses < 'repo > { type Raw = * mut raw :: git_status_list ; unsafe fn from_raw (raw : * mut raw :: git_status_list) -> Statuses < 'repo > { Statuses { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_status_list { self . raw } }
};
}
