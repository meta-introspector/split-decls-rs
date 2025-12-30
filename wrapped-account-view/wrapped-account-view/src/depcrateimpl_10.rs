// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : ? Sized > Drop for Ref < '_ , T > { fn drop (& mut self) { unsafe { * self . state . as_mut () += 1 } ; } }
};
}
