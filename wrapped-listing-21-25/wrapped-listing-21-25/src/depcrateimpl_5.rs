// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Drop for ThreadPool { fn drop (& mut self) { drop (self . sender . take ()) ; for worker in self . workers . drain (..) { println ! ("Shutting down worker {}" , worker . id) ; worker . thread . join () . unwrap () ; } } }
};
}
