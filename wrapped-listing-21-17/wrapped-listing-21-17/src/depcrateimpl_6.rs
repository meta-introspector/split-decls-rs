// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Worker { fn new (id : usize , receiver : mpsc :: Receiver < Job >) -> Worker { let thread = thread :: spawn (| | { receiver ; }) ; Worker { id , thread } } }
};
}
