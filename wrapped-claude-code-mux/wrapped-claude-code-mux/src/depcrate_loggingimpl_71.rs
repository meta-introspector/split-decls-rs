// Generated macro for impl_71 (impl)
macro_rules! Depcrate_loggingimpl_71 {
() => {
// Module: crate::logging
// Provides: {"impl_71"}
// Dependencies: {}
impl Visit for LogVisitor { fn record_debug (& mut self , field : & Field , value : & dyn std :: fmt :: Debug) { if field . name () == "message" { self . message = Some (format ! ("{:?}" , value)) ; } } }
};
}
