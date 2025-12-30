// Generated macro for impl_221 (impl)
macro_rules! Depcrate_task_poolimpl_221 {
() => {
// Module: crate::task_pool
// Provides: {"impl_221"}
// Dependencies: {}
impl < T > TaskPool < T > { pub (crate) fn new_with_threads (sender : Sender < T > , threads : usize) -> TaskPool < T > { TaskPool { sender , pool : Pool :: new (threads) } } pub (crate) fn spawn < F > (& mut self , intent : ThreadIntent , task : F) where F : FnOnce () -> T + Send + UnwindSafe + 'static , T : Send + 'static , { self . pool . spawn (intent , { let sender = self . sender . clone () ; move | | sender . send (task ()) . unwrap () }) } pub (crate) fn spawn_with_sender < F > (& mut self , intent : ThreadIntent , task : F) where F : FnOnce (Sender < T >) + Send + UnwindSafe + 'static , T : Send + 'static , { self . pool . spawn (intent , { let sender = self . sender . clone () ; move | | task (sender) }) } pub (crate) fn len (& self) -> usize { self . pool . len () } }
};
}
