// Generated macro for impl_208 (impl)
macro_rules! Depcrate_ttyimpl_208 {
() => {
// Module: crate::tty
// Provides: {"impl_208"}
// Dependencies: {}
# [doc = " On windows, `GetConsoleMode` will return true if we are in a terminal."] # [doc = " Otherwise false."] # [cfg (windows)] impl < S : AsRawHandle > IsTty for S { fn is_tty (& self) -> bool { let mut mode = 0 ; let ok = unsafe { GetConsoleMode (self . as_raw_handle () as * mut _ , & mut mode) } ; ok == 1 } }
};
}
