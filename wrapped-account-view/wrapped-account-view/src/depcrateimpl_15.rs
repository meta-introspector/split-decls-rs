// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < T : ? Sized > Drop for RefMut < '_ , T > { fn drop (& mut self) { unsafe { * self . state . as_mut () = NOT_BORROWED } ; } }
};
}
