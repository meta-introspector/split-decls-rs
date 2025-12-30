// Generated macro for impl_430 (impl)
macro_rules! Depcrate___macros_writebackimpl_430 {
() => {
// Module: crate::__macros::writeback
// Provides: {"impl_430"}
// Dependencies: {}
impl < T : Message > Drop for WritebackOnDrop < T > { # [inline] fn drop (& mut self) { let new : Option < Retained < T > > = unsafe { Retained :: retain (* self . ptr . as_ptr ()) } ; let _new = ManuallyDrop :: new (new) ; # [cfg (debug_assertions)] if _new . is_none () { panic ! ("found that NULL was written to `&mut Retained<_>`, which is UB! You should handle this with `&mut Option<Retained<_>>` instead") ; } let _ : Retained < T > = unsafe { Retained :: new_nonnull (self . old) } ; } }
};
}
