// Generated macro for NSRunningApplication (trait)
macro_rules! Depcrate_appkitNSRunningApplication {
() => {
// Module: crate::appkit
// Provides: {"NSRunningApplication"}
// Dependencies: {}
pub trait NSRunningApplication : Sized { unsafe fn currentApplication (_ : Self) -> id { msg_send ! [class ! (NSRunningApplication) , currentApplication] } unsafe fn activateWithOptions_ (self , options : NSApplicationActivationOptions) -> BOOL ; unsafe fn runningApplicationWithProcessIdentifier (_ : Self , pid : libc :: pid_t) -> id { msg_send ! [class ! (NSRunningApplication) , runningApplicationWithProcessIdentifier : pid] } }
};
}
