// Generated macro for resize_console (function)
macro_rules! Depcrate_processresize_console {
() => {
// Module: crate::process
// Provides: {"resize_console"}
// Dependencies: {}
fn resize_console (console : HPCON , x : i16 , y : i16) -> Result < () , Error > { unsafe { ResizePseudoConsole (console , COORD { X : x , Y : y }) } ? ; Ok (()) }
};
}
