// Generated macro for impl_37 (impl)
macro_rules! Depcrate_slotimpl_37 {
() => {
// Module: crate::slot
// Provides: {"impl_37"}
// Dependencies: {}
impl < T , F > FnBox < T > for F where F : FnOnce (& Slot < T >) + Send + 'static , T : 'static , { fn call_box (self : Box < F > , other : & Slot < T >) { (* self) (other) } }
};
}
