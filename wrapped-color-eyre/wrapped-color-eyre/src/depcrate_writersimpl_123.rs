// Generated macro for impl_123 (impl)
macro_rules! Depcrate_writersimpl_123 {
() => {
// Module: crate::writers
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a , H : ? Sized , W > HeaderWriter < 'a , H , W > { pub (crate) fn ready (& mut self) -> ReadyHeaderWriter < 'a , '_ , H , W > { self . started = false ; ReadyHeaderWriter (self) } pub (crate) fn in_progress (& mut self) -> ReadyHeaderWriter < 'a , '_ , H , W > { self . started = true ; ReadyHeaderWriter (self) } }
};
}
