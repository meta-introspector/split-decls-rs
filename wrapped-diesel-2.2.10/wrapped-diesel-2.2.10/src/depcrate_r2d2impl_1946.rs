// Generated macro for impl_1946 (impl)
macro_rules! Depcrate_r2d2impl_1946 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1946"}
// Dependencies: {}
impl < Changes , Output , M > crate :: query_dsl :: UpdateAndFetchResults < Changes , Output > for PooledConnection < M > where M : ManageConnection , M :: Connection : crate :: query_dsl :: UpdateAndFetchResults < Changes , Output > , Self : Connection , { fn update_and_fetch (& mut self , changeset : Changes) -> QueryResult < Output > { (* * self) . update_and_fetch (changeset) } }
};
}
