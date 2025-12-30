// Generated macro for impl_1633 (impl)
macro_rules! Depcrate_query_dsl_load_dslimpl_1633 {
() => {
// Module: crate::query_dsl::load_dsl
// Provides: {"impl_1633"}
// Dependencies: {}
impl < Conn , DB , T > ExecuteDsl < Conn , DB > for T where Conn : Connection < Backend = DB > , DB : Backend , T : QueryFragment < DB > + QueryId , { fn execute (query : T , conn : & mut Conn) -> Result < usize , Error > { conn . execute_returning_count (& query) } }
};
}
