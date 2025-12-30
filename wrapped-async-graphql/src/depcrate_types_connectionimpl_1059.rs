// Generated macro for impl_1059 (impl)
macro_rules! Depcrate_types_connectionimpl_1059 {
() => {
// Module: crate::types::connection
// Provides: {"impl_1059"}
// Dependencies: {}
impl ConnectionNameType for DefaultConnectionName { fn type_name < T : OutputType > () -> String { format ! ("{}Connection" , T :: type_name ()) } }
};
}
