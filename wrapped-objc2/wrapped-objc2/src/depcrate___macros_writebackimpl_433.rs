// Generated macro for impl_433 (impl)
macro_rules! Depcrate___macros_writebackimpl_433 {
() => {
// Module: crate::__macros::writeback
// Provides: {"impl_433"}
// Dependencies: {}
impl < T : Message > Drop for WritebackOnDropNullable < T > { # [inline] fn drop (& mut self) { let new : Option < Retained < T > > = unsafe { Retained :: retain (* self . ptr . as_ptr ()) } ; let _ = ManuallyDrop :: new (new) ; let _ : Option < Retained < T > > = unsafe { Retained :: from_raw (self . old) } ; } }
};
}
