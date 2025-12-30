// Generated macro for impl_192 (impl)
macro_rules! Depcrate_rt_pathimpl_192 {
() => {
// Module: crate::rt::path
// Provides: {"impl_192"}
// Dependencies: {}
impl Thread { fn explore (& mut self) { if * self == Thread :: Skip { * self = Thread :: Pending ; } } fn is_pending (& self) -> bool { * self == Thread :: Pending } fn is_active (& self) -> bool { * self == Thread :: Active } fn is_enabled (& self) -> bool { ! self . is_disabled () } fn is_disabled (& self) -> bool { * self == Thread :: Disabled } }
};
}
