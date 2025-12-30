// Generated macro for impl_1012 (impl)
macro_rules! Depcrate_remoteimpl_1012 {
() => {
// Module: crate::remote
// Provides: {"impl_1012"}
// Dependencies: {}
impl < 'repo , 'connection , 'cb > Drop for RemoteConnection < 'repo , 'connection , 'cb > { fn drop (& mut self) { drop (self . remote . disconnect ()) ; } }
};
}
