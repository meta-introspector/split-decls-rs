// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { CoInitializeEx (None , COINIT_MULTITHREADED) . ok () ? ; if let Err (result) = Package :: Current () { MessageBoxW (None , w ! ("This sample must be registered (via register.cmd) and launched from Start.") , w ! ("Error") , MB_ICONSTOP | MB_OK ,) ; return Err (result) ; } } let app : IFrameworkViewSource = CoreApp () . into () ; CoreApplication :: Run (& app) ? ; Ok (()) }
};
}
