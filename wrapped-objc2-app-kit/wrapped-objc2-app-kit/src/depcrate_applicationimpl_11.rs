// Generated macro for impl_11 (impl)
macro_rules! Depcrate_applicationimpl_11 {
() => {
// Module: crate::application
// Provides: {"impl_11"}
// Dependencies: {}
impl NSApplication { # [doc = " An entry point to AppKit applications."] # [doc = ""] # [doc = " See [Apple's documentation][apple-doc] for more details."] # [doc = ""] # [doc = " [apple-doc]: https://developer.apple.com/documentation/appkit/nsapplicationmain(_:_:)"] # [doc (alias = "NSApplicationMain")] pub fn main (mtm : MainThreadMarker) -> ! { let _ = mtm ; # [cfg (not (feature = "gnustep-1-7"))] { extern "C" { fn _NSGetArgc () -> * mut c_int ; fn _NSGetArgv () -> * mut * mut * mut c_char ; } let argc = unsafe { * _NSGetArgc () } ; let argv = unsafe { NonNull :: new (* _NSGetArgv ()) . unwrap () . cast () } ; let _ret = unsafe { Self :: __main (argc , argv) } ; # [cfg (feature = "std")] { std :: process :: exit (_ret as i32) } # [cfg (not (feature = "std"))] { unreachable ! ("NSApplicationMain should not have returned") } } # [cfg (feature = "gnustep-1-7")] { unsafe { Self :: __main (0 , NonNull :: dangling ()) } ; unreachable ! () } } }
};
}
