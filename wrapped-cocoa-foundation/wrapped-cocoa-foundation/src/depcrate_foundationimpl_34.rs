// Generated macro for impl_34 (impl)
macro_rules! Depcrate_foundationimpl_34 {
() => {
// Module: crate::foundation
// Provides: {"impl_34"}
// Dependencies: {}
impl NSProcessInfo for id { unsafe fn processName (self) -> id { msg_send ! [self , processName] } unsafe fn systemUptime (self) -> NSTimeInterval { msg_send ! [self , systemUptime] } unsafe fn operatingSystemVersion (self) -> NSOperatingSystemVersion { msg_send ! [self , operatingSystemVersion] } unsafe fn isOperatingSystemAtLeastVersion (self , version : NSOperatingSystemVersion) -> bool { let res : BOOL = msg_send ! [self , isOperatingSystemAtLeastVersion : version] ; res != NO } }
};
}
