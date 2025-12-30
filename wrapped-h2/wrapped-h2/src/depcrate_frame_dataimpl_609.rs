// Generated macro for impl_609 (impl)
macro_rules! Depcrate_frame_dataimpl_609 {
() => {
// Module: crate::frame::data
// Provides: {"impl_609"}
// Dependencies: {}
impl DataFlags { fn load (bits : u8) -> DataFlags { DataFlags (bits & ALL) } fn is_empty (& self) -> bool { self . 0 == 0 } fn is_end_stream (& self) -> bool { self . 0 & END_STREAM == END_STREAM } fn set_end_stream (& mut self) { self . 0 |= END_STREAM } fn unset_end_stream (& mut self) { self . 0 &= ! END_STREAM } fn is_padded (& self) -> bool { self . 0 & PADDED == PADDED } # [cfg (feature = "unstable")] fn set_padded (& mut self) { self . 0 |= PADDED } }
};
}
