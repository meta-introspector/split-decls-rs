// Generated macro for get_in_place_thread_registry (function)
macro_rules! Depcrate_scopeget_in_place_thread_registry {
() => {
// Module: crate::scope
// Provides: {"get_in_place_thread_registry"}
// Dependencies: {}
fn get_in_place_thread_registry (registry : Option < & Arc < Registry > > ,) -> (Option < & WorkerThread > , Option < & Arc < Registry > >) { let thread = unsafe { WorkerThread :: current () . as_ref () } ; if thread . is_none () && registry . is_none () { let global = global_registry () ; (global . current_thread () , Some (global)) } else { (thread , registry) } }
};
}
