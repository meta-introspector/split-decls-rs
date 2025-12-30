// Generated macro for macos (module)
macro_rules! Depcrate_sysmacos {
() => {
// Module: crate::sys
// Provides: {"macos"}
// Dependencies: {}
# [cfg (target_os = "macos")] mod macos { pub enum CGEventTap { } pub type CGEventTapRef = core_foundation :: mach_port :: CFMachPortRef ; pub enum CGEvent { } pub type CGEventRef = * mut CGEvent ; pub enum CGEventSource { } pub type CGEventSourceRef = * mut CGEventSource ; pub enum CGDisplayMode { } pub type CGDisplayModeRef = * mut CGDisplayMode ; }
};
}
