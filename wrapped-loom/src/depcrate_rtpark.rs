// Generated macro for park (function)
macro_rules! Depcrate_rtpark {
() => {
// Module: crate::rt
// Provides: {"park"}
// Dependencies: {}
# [doc = " Marks the current thread as blocked"] pub (crate) fn park (location : Location) { let switch = execution (| execution | { use thread :: State ; let thread = execution . threads . active_id () ; let active = execution . threads . active_mut () ; trace ! (? thread , ? active . state , "park") ; match active . state { State :: Runnable { unparked : true } => { active . set_runnable () ; return false ; } _ => active . set_blocked (location) , } ; execution . threads . active_mut () . set_blocked (location) ; execution . threads . active_mut () . operation = None ; execution . schedule () }) ; if switch { Scheduler :: switch () ; } }
};
}
