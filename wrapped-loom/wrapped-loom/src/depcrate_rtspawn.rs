// Generated macro for spawn (function)
macro_rules! Depcrate_rtspawn {
() => {
// Module: crate::rt
// Provides: {"spawn"}
// Dependencies: {}
pub (crate) fn spawn < F > (stack_size : Option < usize > , f : F) -> crate :: rt :: thread :: Id where F : FnOnce () + 'static , { let id = execution (| execution | execution . new_thread ()) ; trace ! (thread = ? id , "spawn") ; Scheduler :: spawn (stack_size , Box :: new (move | | { f () ; thread_done () ; }) ,) ; id }
};
}
