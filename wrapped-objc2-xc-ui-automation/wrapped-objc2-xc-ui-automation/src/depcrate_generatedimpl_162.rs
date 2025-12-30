// Generated macro for impl_162 (impl)
macro_rules! Depcrate_generatedimpl_162 {
() => {
// Module: crate::generated
// Provides: {"impl_162"}
// Dependencies: {}
impl XCUIScreenshot { extern_methods ! (# [unsafe (method (new))] # [unsafe (method_family = new)] pub fn new (mtm : MainThreadMarker) -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [cfg (feature = "objc2-app-kit")] # [cfg (target_os = "macos")] # [unsafe (method (image))] # [unsafe (method_family = none)] pub fn image (& self) -> Retained < NSImage >; # [doc = " PNG image data of the underlying image."] # [unsafe (method (PNGRepresentation))] # [unsafe (method_family = none)] pub fn PNGRepresentation (& self) -> Retained < NSData >;) ; }
};
}
