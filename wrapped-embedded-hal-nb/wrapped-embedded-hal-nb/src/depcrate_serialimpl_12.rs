// Generated macro for impl_12 (impl)
macro_rules! Depcrate_serialimpl_12 {
() => {
// Module: crate::serial
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : Read < Word > + ? Sized , Word : Copy > Read < Word > for & mut T { # [inline] fn read (& mut self) -> nb :: Result < Word , Self :: Error > { T :: read (self) } }
};
}
