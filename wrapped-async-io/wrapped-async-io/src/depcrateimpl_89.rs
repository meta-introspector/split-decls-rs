// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl Drop for Timer { fn drop (& mut self) { if let (Some (when) , Some ((id , _))) = (self . when , self . id_and_waker . take ()) { Reactor :: get () . remove_timer (when , id) ; } } }
};
}
