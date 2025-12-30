// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl Wake for Inner { # [inline] fn wake (self : Arc < Self >) { self . unpark () ; } # [inline] fn wake_by_ref (self : & Arc < Self >) { self . unpark () ; } }
};
}
