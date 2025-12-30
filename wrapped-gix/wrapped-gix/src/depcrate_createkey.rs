// Generated macro for key (function)
macro_rules! Depcrate_createkey {
() => {
// Module: crate::create
// Provides: {"key"}
// Dependencies: {}
fn key (name : & 'static str) -> section :: ValueName < 'static > { section :: ValueName :: try_from (name) . expect ("valid key name") }
};
}
