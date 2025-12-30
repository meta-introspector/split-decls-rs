// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { CoInitializeEx (None , COINIT_MULTITHREADED) . ok () ? ; SetProcessDpiAwarenessContext (DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) ? ; } let mut window = Window :: new () ? ; window . run () }
};
}
