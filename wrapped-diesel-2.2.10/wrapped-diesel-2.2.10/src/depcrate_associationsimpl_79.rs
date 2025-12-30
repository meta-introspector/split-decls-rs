// Generated macro for impl_79 (impl)
macro_rules! Depcrate_associationsimpl_79 {
() => {
// Module: crate::associations
// Provides: {"impl_79"}
// Dependencies: {}
impl < T : HasTable > HasTable for & T { type Table = T :: Table ; fn table () -> Self :: Table { T :: table () } }
};
}
