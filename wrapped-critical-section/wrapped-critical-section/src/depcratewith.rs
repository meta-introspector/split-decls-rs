// Generated macro for with (function)
macro_rules! Depcratewith {
() => {
// Module: crate
// Provides: {"with"}
// Dependencies: {}
# [doc = " Execute closure `f` in a critical section."] # [doc = ""] # [doc = " Nesting critical sections is allowed. The inner critical sections"] # [doc = " are mostly no-ops since they're already protected by the outer one."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if the given closure `f` panics. In this case"] # [doc = " the critical section is released before unwinding."] # [inline] pub fn with < R > (f : impl FnOnce (CriticalSection) -> R) -> R { struct Guard { state : RestoreState , } impl Drop for Guard { # [inline (always)] fn drop (& mut self) { unsafe { release (self . state) } } } let state = unsafe { acquire () } ; let _guard = Guard { state } ; unsafe { f (CriticalSection :: new ()) } }
};
}
