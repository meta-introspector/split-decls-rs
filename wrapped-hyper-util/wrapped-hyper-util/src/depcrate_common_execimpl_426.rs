// Generated macro for impl_426 (impl)
macro_rules! Depcrate_common_execimpl_426 {
() => {
// Module: crate::common::exec
// Provides: {"impl_426"}
// Dependencies: {}
impl < F > hyper :: rt :: Executor < F > for Exec where F : Future < Output = () > + Send + 'static , { fn execute (& self , fut : F) { Exec :: execute (self , fut) ; } }
};
}
