// Generated macro for impl_1939 (impl)
macro_rules! Depcrate_r2d2impl_1939 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1939"}
// Dependencies: {}
impl < M > SimpleConnection for PooledConnection < M > where M : ManageConnection , M :: Connection : R2D2Connection + Send + 'static , { fn batch_execute (& mut self , query : & str) -> QueryResult < () > { (* * self) . batch_execute (query) } }
};
}
