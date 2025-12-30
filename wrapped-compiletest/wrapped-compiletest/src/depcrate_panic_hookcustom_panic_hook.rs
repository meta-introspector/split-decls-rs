// Generated macro for custom_panic_hook (function)
macro_rules! Depcrate_panic_hookcustom_panic_hook {
() => {
// Module: crate::panic_hook
// Provides: {"custom_panic_hook"}
// Dependencies: {}
fn custom_panic_hook (default_hook : & PanicHook , info : & panic :: PanicHookInfo < '_ >) { let Some (buf) = take_capture_buf () else { default_hook (info) ; return ; } ; let mut out = buf . lock () . unwrap_or_else (| e | e . into_inner ()) ; let thread = thread :: current () . name () . unwrap_or ("(test runner)") . to_owned () ; let location = get_location (info) ; let payload = payload_as_str (info) . unwrap_or ("Box<dyn Any>") ; let backtrace = Backtrace :: capture () ; writeln ! (out , "\nthread '{thread}' panicked at {location}:\n{payload}") . unwrap () ; match backtrace . status () { BacktraceStatus :: Captured => { let bt = trim_backtrace (backtrace . to_string ()) ; write ! (out , "stack backtrace:\n{bt}" ,) . unwrap () ; } BacktraceStatus :: Disabled => { writeln ! (out , "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace" ,) . unwrap () ; } _ => { } } drop (out) ; set_capture_buf (buf) ; }
};
}
