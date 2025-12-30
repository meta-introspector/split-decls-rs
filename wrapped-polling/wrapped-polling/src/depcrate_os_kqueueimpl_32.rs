// Generated macro for impl_32 (impl)
macro_rules! Depcrate_os_kqueueimpl_32 {
() => {
// Module: crate::os::kqueue
// Provides: {"impl_32"}
// Dependencies: {}
impl EventExtra { # [doc = " Create a new, empty version of this struct."] # [inline] pub const fn empty () -> EventExtra { EventExtra } # [doc = " Set the interrupt flag."] # [inline] pub fn set_hup (& mut self , _value : bool) { } # [doc = " Set the priority flag."] # [inline] pub fn set_pri (& mut self , _value : bool) { } # [doc = " Is the interrupt flag set?"] # [inline] pub fn is_hup (& self) -> bool { false } # [doc = " Is the priority flag set?"] # [inline] pub fn is_pri (& self) -> bool { false } # [inline] pub fn is_connect_failed (& self) -> Option < bool > { None } # [inline] pub fn is_err (& self) -> Option < bool > { None } }
};
}
