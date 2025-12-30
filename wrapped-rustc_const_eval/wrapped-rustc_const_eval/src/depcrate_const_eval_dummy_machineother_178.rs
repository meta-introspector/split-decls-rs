// Generated macro for other_178 (other)
macro_rules! Depcrate_const_eval_dummy_machineother_178 {
() => {
// Module: crate::const_eval::dummy_machine
// Provides: {"other_178"}
// Dependencies: {}
# [doc = " Macro for machine-specific `InterpError` without allocation."] # [doc = " (These will never be shown to the user, but they help diagnose ICEs.)"] pub macro throw_machine_stop_str ($ ($ tt : tt) *) { { # [derive (Debug)] struct Zst ; impl std :: fmt :: Display for Zst { fn fmt (& self , f : & mut std :: fmt :: Formatter <'_ >) -> std :: fmt :: Result { write ! (f , $ ($ tt) *) } } impl rustc_middle :: mir :: interpret :: MachineStopType for Zst { fn diagnostic_message (& self) -> rustc_errors :: DiagMessage { self . to_string () . into () } fn add_args (self : Box < Self >, _ : & mut dyn FnMut (rustc_errors :: DiagArgName , rustc_errors :: DiagArgValue) ,) { } } throw_machine_stop ! (Zst) } }
};
}
