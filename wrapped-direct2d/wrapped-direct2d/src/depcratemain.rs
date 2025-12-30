// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { CoInitializeEx (None , COINIT_MULTITHREADED) . ok () ? ; } let mut window = Window :: new () ? ; window . run () }
};
}
