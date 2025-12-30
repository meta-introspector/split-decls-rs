// Generated macro for impl_644 (impl)
macro_rules! Depcrate_diagnosticsimpl_644 {
() => {
// Module: crate::diagnostics
// Provides: {"impl_644"}
// Dependencies: {}
impl MachineStopType for TerminationInfo { fn diagnostic_message (& self) -> DiagMessage { self . to_string () . into () } fn add_args (self : Box < Self > , _ : & mut dyn FnMut (std :: borrow :: Cow < 'static , str > , rustc_errors :: DiagArgValue) ,) { } }
};
}
