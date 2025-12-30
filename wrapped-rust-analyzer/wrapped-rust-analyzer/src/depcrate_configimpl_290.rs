// Generated macro for impl_290 (impl)
macro_rules! Depcrate_configimpl_290 {
() => {
// Module: crate::config
// Provides: {"impl_290"}
// Dependencies: {}
impl HoverActionsConfig { pub const NO_ACTIONS : Self = Self { implementations : false , references : false , run : false , debug : false , update_test : false , goto_type_def : false , } ; pub fn any (& self) -> bool { self . implementations || self . references || self . runnable () || self . goto_type_def } pub fn none (& self) -> bool { ! self . any () } pub fn runnable (& self) -> bool { self . run || self . debug || self . update_test } }
};
}
