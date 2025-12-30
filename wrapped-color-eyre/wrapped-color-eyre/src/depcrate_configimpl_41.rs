// Generated macro for impl_41 (impl)
macro_rules! Depcrate_configimpl_41 {
() => {
// Module: crate::config
// Provides: {"impl_41"}
// Dependencies: {}
impl PanicHook { pub (crate) fn format_backtrace < 'a > (& 'a self , trace : & 'a backtrace :: Backtrace ,) -> BacktraceFormatter < 'a > { BacktraceFormatter { filters : & self . filters , inner : trace , theme : self . theme , } } # [cfg (feature = "capture-spantrace")] fn spantrace_capture_enabled (& self) -> bool { std :: env :: var ("RUST_SPANTRACE") . map (| val | val != "0") . unwrap_or (self . capture_span_trace_by_default) } # [doc = " Install self as a global panic hook via `std::panic::set_hook`."] pub fn install (self) { std :: panic :: set_hook (self . into_panic_hook ()) ; } # [doc = " Convert self into the type expected by `std::panic::set_hook`."] pub fn into_panic_hook (self ,) -> Box < dyn Fn (& std :: panic :: PanicInfo < '_ >) + Send + Sync + 'static > { Box :: new (move | panic_info | { eprintln ! ("{}" , self . panic_report (panic_info)) ; }) } # [doc = " Construct a panic reporter which prints it's panic report via the"] # [doc = " `Display` trait."] pub fn panic_report < 'a > (& 'a self , panic_info : & 'a std :: panic :: PanicInfo < '_ > ,) -> PanicReport < 'a > { let v = panic_verbosity () ; let capture_bt = v != Verbosity :: Minimal ; # [cfg (feature = "capture-spantrace")] let span_trace = if self . spantrace_capture_enabled () { Some (tracing_error :: SpanTrace :: capture ()) } else { None } ; let backtrace = if capture_bt { Some (backtrace :: Backtrace :: new ()) } else { None } ; PanicReport { panic_info , # [cfg (feature = "capture-spantrace")] span_trace , backtrace , hook : self , } } }
};
}
