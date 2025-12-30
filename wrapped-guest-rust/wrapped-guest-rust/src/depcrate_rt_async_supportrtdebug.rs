// Generated macro for rtdebug (macro)
macro_rules! Depcrate_rt_async_supportrtdebug {
() => {
// Module: crate::rt::async_support
// Provides: {"rtdebug"}
// Dependencies: {}
macro_rules ! rtdebug { ($ ($ f : tt) *) => { if false { std :: eprintln ! ($ ($ f) *) ; } } }
};
}
