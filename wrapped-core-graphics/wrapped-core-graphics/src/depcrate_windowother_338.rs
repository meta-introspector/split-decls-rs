// Generated macro for other_338 (other)
macro_rules! Depcrate_windowother_338 {
() => {
// Module: crate::window
// Provides: {"other_338"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { pub static kCGWindowNumber : CFStringRef ; pub static kCGWindowStoreType : CFStringRef ; pub static kCGWindowLayer : CFStringRef ; pub static kCGWindowBounds : CFStringRef ; pub static kCGWindowSharingState : CFStringRef ; pub static kCGWindowAlpha : CFStringRef ; pub static kCGWindowOwnerPID : CFStringRef ; pub static kCGWindowMemoryUsage : CFStringRef ; pub static kCGWindowWorkspace : CFStringRef ; pub static kCGWindowOwnerName : CFStringRef ; pub static kCGWindowName : CFStringRef ; pub static kCGWindowIsOnscreen : CFStringRef ; pub static kCGWindowBackingLocationVideoMemory : CFStringRef ; pub fn CGWindowListCopyWindowInfo (option : CGWindowListOption , relativeToWindow : CGWindowID ,) -> CFArrayRef ; pub fn CGWindowListCreate (option : CGWindowListOption , relativeToWindow : CGWindowID ,) -> CFArrayRef ; pub fn CGWindowListCreateDescriptionFromArray (windowArray : CFArrayRef) -> CFArrayRef ; pub fn CGWindowListCreateImage (screenBounds : CGRect , listOption : CGWindowListOption , windowID : CGWindowID , imageOption : CGWindowImageOption ,) -> * mut sys :: CGImage ; pub fn CGWindowListCreateImageFromArray (screenBounds : CGRect , windowArray : CFArrayRef , imageOption : CGWindowImageOption ,) -> * mut sys :: CGImage ; }
};
}
