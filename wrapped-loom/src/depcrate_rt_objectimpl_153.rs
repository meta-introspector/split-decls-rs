// Generated macro for impl_153 (impl)
macro_rules! Depcrate_rt_objectimpl_153 {
() => {
// Module: crate::rt::object
// Provides: {"impl_153"}
// Dependencies: {}
impl From < Action > for rt :: atomic :: Action { fn from (action : Action) -> Self { match action { Action :: Atomic (action) => action , _ => unreachable ! () , } } }
};
}
