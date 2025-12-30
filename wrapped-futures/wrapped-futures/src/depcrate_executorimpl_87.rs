// Generated macro for impl_87 (impl)
macro_rules! Depcrate_executorimpl_87 {
() => {
// Module: crate::executor
// Provides: {"impl_87"}
// Dependencies: {}
impl Executor for Limited { fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static { LIMITED . with (| state | state . execute (f)) } fn execute_boxed (& self , f : Box < ExecuteCallback >) { self . execute (| | f . call ()) ; } }
};
}
