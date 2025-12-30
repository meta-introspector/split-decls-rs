// Generated macro for impl_9 (impl)
macro_rules! Depcrate_counterimpl_9 {
() => {
// Module: crate::counter
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > Counter < 'a > { # [doc = " Starts a console time measurement. The measurement"] # [doc = " ends when the constructed `ConsoleTimer` object is dropped."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use gloo_console::Counter;"] # [doc = ""] # [doc = " let _timer = Counter::new(\"foo\");"] # [doc = " ```"] pub fn new (label : & 'a str) -> Counter < 'a > { console :: count_with_label (label) ; Counter { label } } # [doc = " Increments the counter"] pub fn count (& self) { console :: count_with_label (self . label) ; } }
};
}
