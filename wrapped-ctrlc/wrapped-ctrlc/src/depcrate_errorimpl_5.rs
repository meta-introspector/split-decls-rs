// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl Error { fn describe (& self) -> & str { match * self { Error :: NoSuchSignal (_) => "Signal could not be found from the system" , Error :: MultipleHandlers => "Ctrl-C signal handler already registered" , Error :: System (_) => "Unexpected system error" , } } }
};
}
