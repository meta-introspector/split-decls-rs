// Generated macro for impl_798 (impl)
macro_rules! Depcrate_diagnosticsimpl_798 {
() => {
// Module: crate::diagnostics
// Provides: {"impl_798"}
// Dependencies: {}
impl MachineStopType for TerminationInfo { fn diagnostic_message (& self) -> DiagMessage { self . to_string () . into () } fn add_args (self : Box < Self > , _ : & mut dyn FnMut (std :: borrow :: Cow < 'static , str > , rustc_errors :: DiagArgValue) ,) { } }
};
}
