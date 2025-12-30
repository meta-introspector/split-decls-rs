// Generated macro for init_current (function)
macro_rules! Depcrate_threadinit_current {
() => {
// Module: crate::thread
// Provides: {"init_current"}
// Dependencies: {}
fn init_current (execution : & mut Execution , name : Option < String >) -> Thread { let id = execution . threads . active_id () ; let thread = Thread { id : ThreadId { id } , name , } ; execution . threads . local_init (& CURRENT_THREAD_KEY , thread . clone ()) ; thread }
};
}
