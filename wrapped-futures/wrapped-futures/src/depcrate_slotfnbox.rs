// Generated macro for FnBox (trait)
macro_rules! Depcrate_slotFnBox {
() => {
// Module: crate::slot
// Provides: {"FnBox"}
// Dependencies: {}
trait FnBox < T : 'static > : Send + 'static { fn call_box (self : Box < Self > , other : & Slot < T >) ; }
};
}
