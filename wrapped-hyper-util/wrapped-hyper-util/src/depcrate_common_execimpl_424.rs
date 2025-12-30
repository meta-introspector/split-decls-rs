// Generated macro for impl_424 (impl)
macro_rules! Depcrate_common_execimpl_424 {
() => {
// Module: crate::common::exec
// Provides: {"impl_424"}
// Dependencies: {}
impl Exec { pub (crate) fn new < E > (inner : E) -> Self where E : Executor < BoxSendFuture > + Send + Sync + 'static , { Exec :: Executor (Arc :: new (inner)) } pub (crate) fn execute < F > (& self , fut : F) where F : Future < Output = () > + Send + 'static , { match * self { Exec :: Executor (ref e) => { e . execute (Box :: pin (fut)) ; } } } }
};
}
