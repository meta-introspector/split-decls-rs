// Generated macro for pipe (function)
macro_rules! Depcrate_processpipe {
() => {
// Module: crate::process
// Provides: {"pipe"}
// Dependencies: {}
fn pipe () -> win :: Result < (HANDLE , HANDLE) > { let mut p_in = HANDLE :: default () ; let mut p_out = HANDLE :: default () ; unsafe { CreatePipe (& mut p_in , & mut p_out , None , 0) ? } ; Ok ((p_in , p_out)) }
};
}
