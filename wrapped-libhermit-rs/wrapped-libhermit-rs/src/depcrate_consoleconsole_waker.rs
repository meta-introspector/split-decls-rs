// Generated macro for CONSOLE_WAKER (static)
macro_rules! Depcrate_consoleCONSOLE_WAKER {
() => {
// Module: crate::console
// Provides: {"CONSOLE_WAKER"}
// Dependencies: {}
pub (crate) static CONSOLE_WAKER : InterruptTicketMutex < WakerRegistration > = InterruptTicketMutex :: new (WakerRegistration :: new ()) ;
};
}
