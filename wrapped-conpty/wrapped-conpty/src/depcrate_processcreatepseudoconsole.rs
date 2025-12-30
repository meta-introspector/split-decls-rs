// Generated macro for createPseudoConsole (function)
macro_rules! Depcrate_processcreatePseudoConsole {
() => {
// Module: crate::process
// Provides: {"createPseudoConsole"}
// Dependencies: {}
fn createPseudoConsole (size : COORD) -> win :: Result < (HPCON , HANDLE , HANDLE) > { let (pty_in , con_writer) = pipe () ? ; let (con_reader , pty_out) = pipe () ? ; let console = unsafe { CreatePseudoConsole (size , pty_in , pty_out , 0) ? } ; unsafe { CloseHandle (pty_in) ? ; CloseHandle (pty_out) ? ; } Ok ((console , con_reader , con_writer)) }
};
}
