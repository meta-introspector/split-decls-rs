// Generated macro for NSProcessInfo (trait)
macro_rules! Depcrate_foundationNSProcessInfo {
() => {
// Module: crate::foundation
// Provides: {"NSProcessInfo"}
// Dependencies: {}
pub trait NSProcessInfo : Sized { unsafe fn processInfo (_ : Self) -> id { msg_send ! [class ! (NSProcessInfo) , processInfo] } unsafe fn systemUptime (self) -> NSTimeInterval ; unsafe fn processName (self) -> id ; unsafe fn operatingSystemVersion (self) -> NSOperatingSystemVersion ; unsafe fn isOperatingSystemAtLeastVersion (self , version : NSOperatingSystemVersion) -> bool ; }
};
}
