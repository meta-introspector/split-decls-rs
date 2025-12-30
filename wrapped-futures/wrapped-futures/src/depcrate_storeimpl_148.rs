// Generated macro for impl_148 (impl)
macro_rules! Depcrate_storeimpl_148 {
() => {
// Module: crate::store
// Provides: {"impl_148"}
// Dependencies: {}
impl < T , E > Future for Store < T , E > where T : Any + Send + 'static , E : Send + 'static , { type Item = TaskData < T > ; type Error = E ; fn poll (& mut self , task : & mut Task) -> Poll < TaskData < T > , E > { let item = self . item . take () . expect ("cannot poll Store twice") ; Poll :: Ok (task . insert (item)) } fn schedule (& mut self , task : & mut Task) { task . notify () } }
};
}
