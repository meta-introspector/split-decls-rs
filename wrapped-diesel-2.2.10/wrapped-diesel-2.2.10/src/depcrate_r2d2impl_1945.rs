// Generated macro for impl_1945 (impl)
macro_rules! Depcrate_r2d2impl_1945 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1945"}
// Dependencies: {}
impl < M > crate :: migration :: MigrationConnection for PooledConnection < M > where M : ManageConnection , M :: Connection : crate :: migration :: MigrationConnection , Self : Connection , { fn setup (& mut self) -> QueryResult < usize > { (* * self) . setup () } }
};
}
