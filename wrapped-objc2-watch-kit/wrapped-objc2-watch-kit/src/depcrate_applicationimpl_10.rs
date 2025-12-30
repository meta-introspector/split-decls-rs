// Generated macro for impl_10 (impl)
macro_rules! Depcrate_applicationimpl_10 {
() => {
// Module: crate::application
// Provides: {"impl_10"}
// Dependencies: {}
impl WKApplication { # [doc = " The entry point to WatchKit applications."] # [doc = ""] # [doc = " Creates the application object and the application delegate, and sets"] # [doc = " up the app’s event cycle."] # [doc = ""] # [doc = " See [Apple's documentation][apple-doc] for more details."] # [doc = ""] # [doc = " [apple-doc]: https://developer.apple.com/documentation/appkit/nsapplicationmain(_:_:)"] # [doc (alias = "WKApplicationMain")] pub fn main (application_delegate_class_name : Option < & NSString > , mtm : MainThreadMarker) -> ! { let _ = mtm ; let argc = unsafe { * _NSGetArgc () } ; let argv = unsafe { NonNull :: new (* _NSGetArgv ()) . unwrap () . cast () } ; let _ret = unsafe { Self :: __main (argc , argv , application_delegate_class_name) } ; # [cfg (feature = "std")] { std :: process :: exit (_ret as i32) } # [cfg (not (feature = "std"))] { unreachable ! ("WKApplicationMain should not have returned") } } }
};
}
