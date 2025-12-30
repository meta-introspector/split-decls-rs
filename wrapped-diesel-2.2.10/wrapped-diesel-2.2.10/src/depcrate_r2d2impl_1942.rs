// Generated macro for impl_1942 (impl)
macro_rules! Depcrate_r2d2impl_1942 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1942"}
// Dependencies: {}
impl < B , M > LoadConnection < B > for PooledConnection < M > where M : ManageConnection , M :: Connection : LoadConnection < B > + R2D2Connection , { type Cursor < 'conn , 'query > = < M :: Connection as LoadConnection < B > > :: Cursor < 'conn , 'query > ; type Row < 'conn , 'query > = < M :: Connection as LoadConnection < B > > :: Row < 'conn , 'query > ; fn load < 'conn , 'query , T > (& 'conn mut self , source : T ,) -> QueryResult < Self :: Cursor < 'conn , 'query > > where T : Query + QueryFragment < Self :: Backend > + QueryId + 'query , Self :: Backend : QueryMetadata < T :: SqlType > , { (* * self) . load (source) } }
};
}
