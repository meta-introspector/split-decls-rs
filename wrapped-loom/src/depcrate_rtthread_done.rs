// Generated macro for thread_done (function)
macro_rules! Depcrate_rtthread_done {
() => {
// Module: crate::rt
// Provides: {"thread_done"}
// Dependencies: {}
pub fn thread_done () { let locals = execution (| execution | { let thread = execution . threads . active_id () ; trace ! (? thread , "thread_done: drop locals") ; execution . threads . active_mut () . drop_locals () }) ; drop (locals) ; execution (| execution | { let thread = execution . threads . active_id () ; execution . threads . active_mut () . operation = None ; execution . threads . active_mut () . set_terminated () ; let switch = execution . schedule () ; trace ! (? thread , ? switch , "thread_done: terminate") ; }) ; }
};
}
