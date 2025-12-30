// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl Drop for ChildGuard { fn drop (& mut self) { if self . kill_on_drop { self . get_mut () . kill () . ok () ; } if self . reap_on_drop { self . inner . reap (& self . reaper . sys) ; } self . reaper . child_count . fetch_sub (1 , Ordering :: Acquire) ; } }
};
}
