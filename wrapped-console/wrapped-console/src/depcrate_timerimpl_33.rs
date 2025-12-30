// Generated macro for impl_33 (impl)
macro_rules! Depcrate_timerimpl_33 {
() => {
// Module: crate::timer
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a > Timer < 'a > { # [doc = " Starts a console time measurement. The measurement"] # [doc = " ends when the constructed `ConsoleTimer` object is dropped."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use gloo_console::Timer;"] # [doc = ""] # [doc = " let _timer = Timer::new(\"foo\");"] # [doc = " ```"] pub fn new (label : & 'a str) -> Timer < 'a > { console :: time_with_label (label) ; Timer { label } } # [doc = " Starts a scoped console time measurement"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use gloo_console::Timer;"] # [doc = ""] # [doc = " let value = Timer::scope(\"foo\", || {"] # [doc = "     // Code to measure here"] # [doc = " });"] # [doc = " ```"] pub fn scope < F , T > (label : & str , f : F) -> T where F : FnOnce () -> T , { let _timer = Timer :: new (label) ; f () } }
};
}
