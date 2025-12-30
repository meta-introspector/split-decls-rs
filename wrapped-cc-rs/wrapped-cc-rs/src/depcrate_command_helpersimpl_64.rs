// Generated macro for impl_64 (impl)
macro_rules! Depcrate_command_helpersimpl_64 {
() => {
// Module: crate::command_helpers
// Provides: {"impl_64"}
// Dependencies: {}
impl CargoOutput { pub (crate) fn new () -> Self { # [allow (clippy :: disallowed_methods)] Self { metadata : true , warnings : true , output : OutputKind :: Forward , debug : match std :: env :: var_os ("CC_ENABLE_DEBUG_OUTPUT") { Some (v) => v != "0" && v != "false" && ! v . is_empty () , None => false , } , checked_dbg_var : Arc :: new (AtomicBool :: new (false)) , } } pub (crate) fn print_metadata (& self , s : & dyn Display) { if self . metadata { println ! ("{s}") ; } } pub (crate) fn print_warning (& self , arg : & dyn Display) { if self . warnings { println ! ("cargo:warning={arg}") ; } } pub (crate) fn print_debug (& self , arg : & dyn Display) { if self . metadata && self . checked_dbg_var . compare_exchange (false , true , Ordering :: Relaxed , Ordering :: Relaxed) . is_ok () { println ! ("cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT") ; } if self . debug { println ! ("{arg}") ; } } fn stdio_for_warnings (& self) -> Stdio { if self . warnings { Stdio :: piped () } else { Stdio :: null () } } fn stdio_for_output (& self) -> Stdio { match self . output { OutputKind :: Capture => Stdio :: piped () , OutputKind :: Forward => Stdio :: inherit () , OutputKind :: Discard => Stdio :: null () , } } }
};
}
