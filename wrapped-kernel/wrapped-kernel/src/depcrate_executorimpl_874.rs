// Generated macro for impl_874 (impl)
macro_rules! Depcrate_executorimpl_874 {
() => {
// Module: crate::executor
// Provides: {"impl_874"}
// Dependencies: {}
impl Wake for TaskNotify { fn wake (self : Arc < Self >) { self . wake_by_ref () ; } fn wake_by_ref (self : & Arc < Self >) { let _ = futex_wake_or_set (& self . futex , 1 , u32 :: MAX) ; } }
};
}
