// Generated macro for impl_154 (impl)
macro_rules! Depcrate_rt_objectimpl_154 {
() => {
// Module: crate::rt::object
// Provides: {"impl_154"}
// Dependencies: {}
impl From < Action > for rt :: mpsc :: Action { fn from (action : Action) -> Self { match action { Action :: Channel (action) => action , _ => unreachable ! () , } } }
};
}
