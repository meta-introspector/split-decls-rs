// Generated macro for impl_243 (impl)
macro_rules! Depcrate_writer_file_helpersimpl_243 {
() => {
// Module: crate::writer::file::helpers
// Provides: {"impl_243"}
// Dependencies: {}
impl < T > PushPos < T > for Vec < T > { fn push_pos (& mut self , value : T) -> u32 { self . push (value) ; (self . len () - 1) . try_into () . unwrap () } }
};
}
